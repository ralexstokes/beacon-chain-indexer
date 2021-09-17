use crate::chain_source::{ChainSource, LighthouseStore};
use rusty_leveldb::{Options, DB};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Default)]
pub struct Builder {
    index_db: Option<PathBuf>,
    lighthouse_db: Option<PathBuf>,
    start_slot: u64,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            index_db: None,
            lighthouse_db: None,
            start_slot: 0,
        }
    }

    pub fn with_index_db<P: Into<PathBuf>>(mut self, db: P) -> Self {
        self.index_db = Some(db.into());
        self
    }

    pub fn from_lighthouse<P: Into<PathBuf>>(mut self, db: P) -> Self {
        self.lighthouse_db = Some(db.into());
        self
    }

    // The indexer will ignore any data older than `slot`.
    pub fn with_start_slot(mut self, slot: u64) -> Self {
        self.start_slot = slot;
        self
    }
}

impl Builder {
    pub fn build(self) -> Indexer<LighthouseStore> {
        let options = Options::default();
        let db_path = self.index_db.expect("provided path");
        let db = DB::open(db_path, options).expect("can open index db");

        let node_db_path = self.lighthouse_db.expect("provided node path");
        let chain = LighthouseStore::new(node_db_path);

        let start_slot = self.start_slot;
        Indexer {
            db,
            chain,
            start_slot,
        }
    }
}

#[derive(Error, Debug)]
pub enum IndexerError {
    #[error("missing epoch {0} in index")]
    MissingEpoch(u64),
    #[error("missing slot {0} in index")]
    MissingSlot(u64),
}

pub struct Indexer<C: ChainSource> {
    db: DB,
    chain: C,
    start_slot: u64,
}

impl<C: ChainSource> Indexer<C> {
    pub fn run(&mut self) {
        unimplemented!()
    }

    pub fn get_attestation_participation(&self, epoch: u64) -> Result<f64, IndexerError> {
        unimplemented!()
    }

    pub fn get_sync_committee_participation(&self, slot: u64) -> Result<f64, IndexerError> {
        unimplemented!()
    }
}
