#![no_std]

extern crate alloc;
pub mod bindings;

use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::fmt::{Display, Formatter};
use filesystem::path::Path;
use filesystem::FileSystem;

#[derive(Clone)]
pub enum Value {
    String(Arc<str>),
    Integer(i64),
    Float(f64),
    Blob(Arc<[u8]>),
    Null,
}

impl Value {
    pub fn as_string(&self) -> Option<Arc<str>> {
        if let Value::String(s) = self {
            Some(s.clone())
        } else {
            None
        }
    }

    pub fn as_integer(&self) -> Option<i64> {
        if let Value::Integer(i) = self {
            Some(*i)
        } else {
            None
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        if let Value::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }

    pub fn as_blob(&self) -> Option<Arc<[u8]>> {
        if let Value::Blob(b) = self {
            Some(b.clone())
        } else {
            None
        }
    }

    pub fn as_null(&self) -> Option<()> {
        if let Value::Null = self {
            Some(())
        } else {
            None
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Value::String(value) => write!(f, "{}", value),
            Value::Integer(value) => write!(f, "{}", value),
            Value::Float(value) => write!(f, "{}", value),
            Value::Blob(value) => write!(f, "{}", String::from_utf8_lossy(value)),
            Value::Null => write!(f, "null"),
        }
    }
}

pub trait Database: DatabaseReader {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self, i32>
    where
        Self: Sized;
}

pub trait DatabaseReader {
    type Iter: Iterator<Item = Self::Record>;
    type Record: TableRecord;

    fn read_table<S>(&self, table_name: S) -> Option<Self::Iter>
    where
        S: AsRef<str>;
}

pub trait DatabaseExt: Database {
    fn from_path<R, F, P>(fs: R, path: P) -> Result<Self, i32>
    where
        R: AsRef<F>,
        F: FileSystem,
        P: AsRef<Path>,
        Self: Sized;
}

impl<T: Database> DatabaseExt for T {
    fn from_path<R, F, P>(fs: R, path: P) -> Result<Self, i32>
    where
        R: AsRef<F>,
        F: FileSystem,
        P: AsRef<Path>,
        Self: Sized
    {
        let data = fs.as_ref().read_file(path.as_ref()).map_err(|e| e as i32)?;
        Self::from_bytes(data)
    }
}

pub trait TableRecord {
    fn get_value(&self, key: usize) -> Option<Value>;
}
