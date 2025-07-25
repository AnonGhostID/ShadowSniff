#![no_std]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use collector::Collector;

pub enum SendError {
    Network,
    UnsupportedLogFile,
    LogFileTooBig
}

/// Represents a log file to be sent or processed.
pub enum LogFile {
    /// A tuple containing:
    /// - A URL pointing to a `.zip` log archive.
    /// - The size of the log file in bytes.
    ExternalLink((String, usize)),

    /// The raw bytes of a `.zip` log archive.
    ZipArchive(Vec<u8>)
}

/// A trait for sending log files to a destination service.
pub trait LogSender {
    /// Sends a log file to the destination service.
    ///
    /// # Parameters
    ///
    /// - `zip_archive`: A [`LogFile`] enum representing the log file to send.
    /// - `password`: An [`Option<String>`] that specifies the password for the archive, if it is password-protected.
    /// - `collector`: A type that implements the [`Collector`] trait, providing log-related metadata or additional context.
    ///
    /// # Returns
    ///
    /// - `Result<(), SendError>`: Returns `Ok(())` if the log was sent successfully, or a [`SendError`] if the operation failed.
    fn send<C>(&self, zip_archive: LogFile, password: Option<String>, collector: C) -> Result<(), SendError>
    where
        C: Collector;
}
