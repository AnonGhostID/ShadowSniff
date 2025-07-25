#![no_std]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use collector::Collector;

pub enum SendError {
    NetworkError
}

/// A trait for sending log files to a destination service.
pub trait LogSender {
    /// Sends a zipped log archive to the destination service.
    ///
    /// # Parameters
    ///
    /// - `zip_archive`: A [`Vec<u8>`] representing the bytes of the `.zip` archive containing the log files.
    /// - `password`: An [`Option<String>`] that specifies the password for the archive, if it is password-protected.
    /// - `collector`: A type that implements the [`Collector`] trait, providing log-related metadata or additional context.
    ///
    /// # Returns
    ///
    /// - `Result<(), SendError>`: Returns `Ok(())` if the log was sent successfully, or a [`SendError`] if the operation failed.
    fn send<C>(&self, zip_archive: Vec<u8>, password: Option<String>, collector: C) -> Result<(), SendError>
    where
        C: Collector;
}
