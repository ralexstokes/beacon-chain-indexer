use crate::level_db::{BytesKey, Database};
use db_key::Key;
use eth2::types::Hash256;
use leveldb::iterator::LevelDBIterator;
use std::convert::TryInto;
use std::path::PathBuf;

// pub const DEFAULT_SLOTS_PER_RESTORE_POINT: u64 = 2048;

const SPLIT_COL: &'static str = "bma";
const SPLIT_KEY: Hash256 = Hash256::repeat_byte(2);
const BEACON_BLOCK_ROOTS_COL: &'static str = "bbr";
const ROOT_SIZE: usize = 32;

pub struct LighthouseStore {
    hot_db: Database,
    cold_db: Database,
    split_slot: u64,
}

fn read_split_slot(db: &mut Database) -> u64 {
    let mut key = SPLIT_COL.as_bytes().to_vec();
    key.extend(SPLIT_KEY.to_fixed_bytes());
    let result = db.get(&key).expect("can read");
    let slot_data = &result[0..8];
    u64::from_le_bytes(slot_data.try_into().expect("is correct"))
}

fn read_block_roots(db: &mut Database) -> Vec<Hash256> {
    let mut roots = vec![];

    let iter = db.iter();

    let start_key = BytesKey::from_u8(BEACON_BLOCK_ROOTS_COL.as_bytes());
    iter.seek(&start_key);

    for (key, batch) in iter {
        let key_data = key.as_slice(|data| data.to_vec());
        if !key_data.starts_with(BEACON_BLOCK_ROOTS_COL.as_bytes()) {
            break;
        }
        for root in batch.chunks(ROOT_SIZE) {
            roots.push(Hash256::from_slice(root));
        }
    }

    roots
}

impl LighthouseStore {
    pub fn new<P: Into<PathBuf>>(beacon_data_dir_path: P) -> Self {
        let data_dir_path = beacon_data_dir_path.into();
        let mut cold_path = data_dir_path.clone();
        cold_path.push("beacon");
        cold_path.push("freezer_db");
        let cold_db = Database::open(&cold_path);

        let mut hot_path = data_dir_path;
        hot_path.push("beacon");
        hot_path.push("chain_db");
        let mut hot_db = Database::open(&hot_path);

        let split_slot = read_split_slot(&mut hot_db);

        Self {
            hot_db,
            cold_db,
            split_slot,
        }
    }

    pub fn run_index(&mut self) {
        let block_roots = read_block_roots(&mut self.cold_db);
        dbg!(block_roots.len());
    }
}
