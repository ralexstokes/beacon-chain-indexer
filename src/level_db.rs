//! Compatiblity for leveldb wrapper, borrowed from: https://github.com/sigp/lighthouse
use db_key::Key;
use leveldb::database::Database as DB;
use leveldb::iterator::{Iterable, Iterator as DBIter};
use leveldb::kv::KV;
use leveldb::options::{Options, ReadOptions, WriteOptions};
use std::path::Path;

/// Used for keying leveldb.
#[derive(Debug, PartialEq)]
pub struct BytesKey {
    key: Vec<u8>,
}

impl Key for BytesKey {
    fn from_u8(key: &[u8]) -> Self {
        Self { key: key.to_vec() }
    }

    fn as_slice<T, F: Fn(&[u8]) -> T>(&self, f: F) -> T {
        f(self.key.as_slice())
    }
}

pub struct Database(DB<BytesKey>);

impl Database {
    pub fn open(p: &Path) -> Self {
        let mut options = Options::new();
        options.create_if_missing = true;
        let inner = DB::open(p, options).expect("can open");
        Self(inner)
    }

    pub fn get(&mut self, key: &[u8]) -> Option<Vec<u8>> {
        let read_opts = ReadOptions::new();
        let key = BytesKey::from_u8(key);
        self.0.get(read_opts, key).expect("can read")
    }

    pub fn put(&mut self, key: &[u8], val: &[u8]) {
        let write_opts = WriteOptions::new();
        let key = BytesKey::from_u8(key);
        self.0.put(write_opts, key, val).expect("can write");
    }

    pub fn iter(&mut self) -> DBIter<BytesKey> {
        let read_opts = ReadOptions::new();
        self.0.iter(read_opts)
    }
}
