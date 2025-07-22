#![no_std]
#![allow(unsafe_op_in_unsafe_fn)]

extern crate alloc;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use collector::Collector;
use core::ffi::c_void;
use core::ptr::null_mut;
use filesystem::FileSystem;
use utils::path::Path;
use windows_sys::Win32::Foundation::CloseHandle;
use windows_sys::Win32::Foundation::HANDLE;
use windows_sys::Win32::Foundation::TRUE;
use windows_sys::Win32::System::Threading::{CreateThread, WaitForMultipleObjects};

#[macro_export]
macro_rules! composite_task {
    ( $( $expr:expr ),* $(,)? ) => {
        CompositeTask::new(vec![
            $(
                alloc::sync::Arc::new($expr)
            ),*
        ])
    };
}

#[macro_export]
macro_rules! parent_name {
    ($name:expr) => {
        fn parent_name(&self) -> Option<alloc::string::String> {
            Some(obfstr::obfstr!($name).to_owned())
        }
    };
}

#[macro_export]
macro_rules! impl_composite_task_runner {
    ($task_type:ty) => {
        impl<C: collector::Collector, F: filesystem::FileSystem> Task<C, F> for $task_type {
            fn run(&self, parent: &utils::path::Path, filesystem: &F, collector: &C) {
                self.inner.run(parent, filesystem, collector);
            }
        }
    };

    ($task_type:ty, $parent_name:expr) => {
        impl<C: collector::Collector, F: filesystem::FileSystem> Task<C, F> for $task_type {
            $crate::parent_name!($parent_name);

            fn run(&self, parent: &utils::path::Path, filesystem: &F, collector: &C) {
                self.inner.run(parent, filesystem, collector);
            }
        }
    };
}

pub trait Task<C: Collector, F: FileSystem>: Send + Sync {
    fn parent_name(&self) -> Option<String> {
        None
    }

    fn run(&self, parent: &Path, filesystem: &F, collector: &C);
}

pub struct CompositeTask<C: Collector, F: FileSystem> {
    subtasks: Vec<Arc<dyn Task<C, F>>>,
}

impl<C: Collector, F: FileSystem> CompositeTask<C, F> {
    pub fn new(subtasks: Vec<Arc<dyn Task<C, F>>>) -> CompositeTask<C, F> {
        Self { subtasks }
    }
}

impl<C: Collector, F: FileSystem> Task<C, F> for CompositeTask<C, F> {
    fn run(&self, parent: &Path, filesystem: &F, collector: &C) {
        match self.subtasks.len() {
            0 => (),
            1 => {
                let task = &self.subtasks[0];
                task.run(&task_path(task, parent), filesystem, collector);
            }
            _ => run_tasks(&self.subtasks, parent, filesystem, collector),
        }
    }
}

fn run_tasks<C, F>(tasks: &[Arc<dyn Task<C, F>>], parent: &Path, filesystem: &F, collector: &C)
where
    C: Collector,
    F: FileSystem,
{
    let mut handles: Vec<HANDLE> = Vec::new();

    for task in tasks {
        let params = Box::new(ThreadParams {
            task: task.clone(),
            path: task_path(task, parent),
            filesystem,
            collector,
        });

        let handle = unsafe {
            CreateThread(
                null_mut(),
                0,
                Some(thread_proc::<C, F>),
                Box::into_raw(params) as *mut _,
                0,
                null_mut(),
            )
        };

        if !handle.is_null() {
            handles.push(handle);
        }
    }

    unsafe {
        WaitForMultipleObjects(handles.len() as _, handles.as_ptr(), TRUE, 0xFFFFFFFF);
    }

    for handle in handles {
        unsafe {
            CloseHandle(handle);
        }
    }
}

fn task_path<C: Collector, F: FileSystem>(task: &Arc<dyn Task<C, F>>, parent: &Path) -> Path {
    match task.parent_name() {
        Some(name) => parent / name,
        None => parent.clone(),
    }
}

struct ThreadParams<'a, C: Collector, F: FileSystem> {
    task: Arc<dyn Task<C, F>>,
    path: Path,
    filesystem: &'a F,
    collector: &'a C,
}

#[allow(unsafe_op_in_unsafe_fn)]
unsafe extern "system" fn thread_proc<C: Collector, F: FileSystem>(param: *mut c_void) -> u32 {
    let params = Box::from_raw(param as *mut ThreadParams<C, F>);

    params
        .task
        .run(&params.path, params.filesystem, params.collector);

    drop(params);

    0
}
