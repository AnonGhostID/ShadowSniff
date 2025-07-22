#![no_std]

extern crate alloc;
pub mod storage;
pub mod virtualfs;

use alloc::vec::Vec;
use core::ops::Deref;
use utils::path::Path;


/// Trait representing a generic file system interface.
pub trait FileSystem {
    /// Reads the entire content of the file at the given path.
    fn read_file(&self, path: &Path) -> Result<Vec<u8>, u32>;

    /// Creates a single directory at the given path.
    fn mkdir(&self, path: &Path) -> Result<(), u32>;

    /// Creates all missing directories in the given path recursively.
    fn mkdirs(&self, path: &Path) -> Result<(), u32>;

    /// Removes all contents inside the directory at the given path.
    fn remove_dir_contents(&self, path: &Path) -> Result<(), u32>;

    /// Removes an empty directory at the given path.
    fn remove_dir(&self, path: &Path) -> Result<(), u32>;

    /// Removes a file at the given path.
    fn remove_file(&self, path: &Path) -> Result<(), u32>;

    /// Creates a new empty file at the given path.
    fn create_file(&self, path: &Path) -> Result<(), u32>;

    /// Writes data to the file at the given path.
    ///
    /// If the parent directories or the file do not exist, they **should be created**.
    fn write_file(&self, path: &Path, data: &[u8]) -> Result<(), u32>;

    /// Lists files under the given directory path, filtered by the provided closure.
    ///
    /// Returns `None` if the directory cannot be listed.
    ///
    /// # Arguments
    ///
    /// * `filter` - A function that takes a file path and returns `true` to include it in the results.
    fn list_files_filtered<F>(&self, path: &Path, filter: &F) -> Option<Vec<Path>>
    where
        F: Fn(&Path) -> bool;

    /// Returns the creation and modification times of the file at the given path, if available.
    ///
    /// The returned tuple is `(creation_time, modification_time)`.
    fn get_filetime(&self, path: &Path) -> Option<(u32, u32)>;

    /// Checks if a file or directory exists at the given path.
    fn is_exists(&self, path: &Path) -> bool;

    /// Checks if the given path is a directory.
    fn is_dir(&self, path: &Path) -> bool;

    /// Checks if the given path is a file.
    fn is_file(&self, path: &Path) -> bool;
}

/// Extension trait providing additional helper methods for file systems.
pub trait FileSystemExt: FileSystem {
    /// Removes a directory and all its contents recursively.
    fn remove_dir_all(&self, path: &Path) -> Result<(), u32>;

    /// Lists all files and directories under the given directory path.
    ///
    /// Returns `None` if the directory cannot be listed.
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

/// Trait for writing data to a file system path.
pub trait WriteTo {
    /// Writes the data to the specified filesystem path.
    fn write_to<F, P>(&self, filesystem: &F, path: P) -> Result<(), u32>
    where
        F: FileSystem,
        P: AsRef<Path>;
}

impl<T> WriteTo for T
where
    T: AsRef<[u8]> + ?Sized,
{
    fn write_to<F, P>(&self, filesystem: &F, path: P) -> Result<(), u32>
    where
        F: FileSystem,
        P: AsRef<Path>,
    {
        filesystem.write_file(path.as_ref(), self.as_ref())
    }
}

/// Copies a single file from `src_fs` at `src_path` to `dst_fs` at `dst_path`.
///
/// If `with_filename` is true, appends the source filename to `dst_path`.
///
/// Creates parent directories in the destination if they do not exist.
pub fn copy_file<SrcFs, DstFs>(
    src_fs: &SrcFs,
    src_path: &Path,
    dst_fs: &DstFs,
    dst_path: &Path,
    with_filename: bool,
) -> Result<(), u32>
where
    SrcFs: FileSystem,
    DstFs: FileSystem,
{
    let dst_path = if with_filename {
        &(dst_path / src_path.fullname().ok_or(2u32)?)
    } else {
        dst_path
    };

    let data = src_fs.read_file(src_path)?;

    if let Some(parent) = dst_path.parent()
        && !dst_fs.is_exists(&parent)
    {
        dst_fs.mkdirs(&parent)?;
    }

    dst_fs.write_file(dst_path, &data)
}

/// Copies a folder recursively from `src_fs` at `src_path` to `dst_fs` at `dst_path`,
/// applying a filter function to select which files/directories to copy.
///
/// The copied folder will be created inside `dst_path` using the folder's own name.
pub fn copy_folder_with_filter<SrcFs, DstFs, F>(
    src_fs: &SrcFs,
    src_path: &Path,
    dst_fs: &DstFs,
    dst_path: &Path,
    filter: &F,
) -> Result<(), u32>
where
    SrcFs: FileSystem,
    DstFs: FileSystem,
    F: Fn(&Path) -> bool,
{
    if !src_fs.is_dir(src_path) {
        return Err(1);
    }

    let dst_path = dst_path / src_path.fullname().ok_or(2u32)?;
    copy_content_with_filter(src_fs, src_path, dst_fs, &dst_path, filter)
}

/// Copies a folder recursively from `src_fs` at `src_path` to `dst_fs` at `dst_path`
/// without any filter (copies everything).
pub fn copy_folder<SrcFs, DstFs>(
    src_fs: &SrcFs,
    src_path: &Path,
    dst_fs: &DstFs,
    dst_path: &Path,
) -> Result<(), u32>
where
    SrcFs: FileSystem,
    DstFs: FileSystem,
{
    copy_folder_with_filter(src_fs, src_path, dst_fs, dst_path, &|_| true)
}

/// Copies all contents of a directory recursively from `src_fs` at `src_path`
/// to `dst_fs` at `dst_path` without filtering.
pub fn copy_content<SrcFs, DstFs>(
    src_fs: &SrcFs,
    src_path: &Path,
    dst_fs: &DstFs,
    dst_path: &Path,
) -> Result<(), u32>
where
    SrcFs: FileSystem,
    DstFs: FileSystem,
{
    copy_content_with_filter(src_fs, src_path, dst_fs, dst_path, &|_| true)
}

/// Copies the contents of a directory recursively from `src_fs` at `src_path`
/// to `dst_fs` at `dst_path`, applying a filter function.
pub fn copy_content_with_filter<SrcFs, DstFs, F>(
    src_fs: &SrcFs,
    src_path: &Path,
    dst_fs: &DstFs,
    dst_path: &Path,
    filter: &F,
) -> Result<(), u32>
where
    SrcFs: FileSystem,
    DstFs: FileSystem,
    F: Fn(&Path) -> bool,
{
    if !src_fs.is_dir(src_path) {
        return Err(1u32);
    }

    if let Some(files) = src_fs.list_files_filtered(src_path, filter) {
        for entry in files {
            let relative = entry.strip_prefix(src_path.deref()).ok_or(2u32)?;
            let new_dst = dst_path / relative;

            if src_fs.is_dir(&entry) {
                copy_content_with_filter(src_fs, &entry, dst_fs, &new_dst, filter)?;
            } else {
                copy_file(src_fs, &entry, dst_fs, &new_dst, false)?;
            }
        }
    }

    Ok(())
}