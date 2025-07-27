use crate::alloc::borrow::ToOwned;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use collector::Collector;
use core::ffi::CStr;
use core::fmt::Write;
use core::ptr::null_mut;
use filesystem::path::Path;
use filesystem::{FileSystem, WriteTo};
use tasks::{parent_name, Task};
use utils::process::get_process_list;
use windows_sys::Win32::Foundation::{CloseHandle, MAX_PATH};
use windows_sys::Win32::System::ProcessStatus::{K32EnumProcesses, K32GetModuleBaseNameA};
use windows_sys::Win32::System::Threading::{
    OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ,
};

pub(super) struct ProcessesTask;

impl<C: Collector, F: FileSystem> Task<C, F> for ProcessesTask {
    parent_name!("Processes.txt");

    fn run(&self, parent: &Path, filesystem: &F, _: &C) {
        let processes = get_process_list();

        let max_pid_width = processes
            .iter()
            .map(|p| p.pid.to_string().len())
            .max()
            .unwrap_or(3);

        let pid_col_width = max_pid_width + 2;

        let mut output = String::new();
        let _ = writeln!(
            &mut output,
            "{:<width$}{}",
            "PID",
            "NAME",
            width = pid_col_width
        );

        for process in processes {
            let _ = writeln!(
                &mut output,
                "{:<width$}{}",
                process.pid,
                process.name,
                width = pid_col_width
            );
        }

        let _ = output.write_to(filesystem, parent);
    }
}
