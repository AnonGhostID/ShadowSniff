#![no_std]

extern crate alloc;
pub mod storage;

use alloc::vec::Vec;
use core::ops::Deref;
use utils::path::Path;

pub trait FileSystem {
    fn read_file(&self, path: &Path) -> Result<Vec<u8>, u32>;

    fn mkdir(&self, path: &Path) -> Result<(), u32>;

    fn mkdirs(&self, path: &Path) -> Result<(), u32>;

    fn remove_dir_contents(&self, path: &Path) -> Result<(), u32>;

    fn remove_dir(&self, path: &Path) -> Result<(), u32>;

    fn remove_file(&self, path: &Path) -> Result<(), u32>;

    fn create_file(&self, path: &Path) -> Result<(), u32>;

    fn write_file(&self, path: &Path, data: &[u8]) -> Result<(), u32>;

    fn list_files_filtered<F>(&self, path: &Path, filter: &F) -> Option<Vec<Path>>
    where
        F: Fn(&Path) -> bool;

    fn get_filetime(&self, path: &Path) -> Option<(u32, u32)>;

    fn is_exists(&self, path: &Path) -> bool;

    fn is_dir(&self, path: &Path) -> bool;

    fn is_file(&self, path: &Path) -> bool;
}

pub trait FileSystemExt: FileSystem {
    fn remove_dir_all(&self, path: &Path) -> Result<(), u32>;

    fn list_files(&self, path: &Path) -> Option<Vec<Path>>;
}

impl<F: FileSystem> FileSystemExt for F {
    fn remove_dir_all(&self, path: &Path) -> Result<(), u32> {
        self.remove_dir_contents(path)?;
        self.remove_dir(path)
    }

    fn list_files(&self, path: &Path) -> Option<Vec<Path>> {
        self.list_files_filtered(path, &|_| true)
    }
}

pub trait WriteTo {
    fn write_to<F, P>(&self, filesystem: &F, path: P) -> Result<(), u32>
    where
        F: FileSystem,
        P: AsRef<Path>;
}

impl<T> WriteTo for T
where
    T: AsRef<[u8]> + ?Sized
{
    fn write_to<F, P>(&self, filesystem: &F, path: P) -> Result<(), u32>
    where
        F: FileSystem,
        P: AsRef<Path>,
    {
        filesystem.write_file(path.as_ref(), self.as_ref())
    }
}