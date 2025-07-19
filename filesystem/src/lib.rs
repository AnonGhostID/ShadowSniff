#![no_std]

pub mod storage;

use alloc::vec::Vec;
use utils::path::Path;

extern crate alloc;

trait FileSystem {
    fn read_file(&self, path: Path) -> Result<Vec<u8>, u32>;

    fn mkdir(&self, path: Path) -> Result<(), u32>;

    fn mkdirs(&self, path: Path) -> Result<(), u32>;

    fn remove_dir_contents(&self, path: Path) -> Result<(), u32>;

    fn remove_dir(&self, path: Path) -> Result<(), u32>;

    fn remove_dir_all(&self, path: Path) -> Result<(), u32>;

    fn remove_file(&self, path: Path) -> Result<(), u32>;

    fn create_file(&self, path: Path) -> Result<(), u32>;

    fn write_file(&self, path: Path, data: &[u8]) -> Result<(), u32>;

    fn list_files(&self, path: Path) -> Option<Vec<Path>> {
        self.list_files_filtered(path, &|_| true)
    }

    fn list_files_filtered<F>(&self, path: Path, filter: &F)
    where
        F: Fn(&Path) -> bool;

    fn get_filetime(&self, path: Path) -> Option<(u32, u32)>;

    fn is_exists(&self, path: Path) -> bool;

    fn is_dir(&self, path: Path) -> bool;

    fn is_file(&self, path: Path) -> bool;
}
