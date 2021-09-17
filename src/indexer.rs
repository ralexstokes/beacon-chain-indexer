use crate::chain_source::LighthouseStore;
use crate::level_db::Database;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Default)]
pub struct Builder {
    index_db: Option<PathBuf>,
    lighthouse_db: Option<PathBuf>,
    start_slot: u64,
    end_slot: u64,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            index_db: None,
            lighthouse_db: None,
            start_slot: 0,
            end_slot: u64::MAX,
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

    pub fn until_slot(mut self, slot: u64) -> Self {
        self.end_slot = slot;
        self
    }

    pub fn build(self) -> Indexer {
        let db_path = self.index_db.expect("provided path");
        let db = Database::open(&db_path);

        let node_db_path = self.lighthouse_db.expect("provided node path");
        let source = LighthouseStore::new(node_db_path);

        let start_slot = self.start_slot;
        let end_slot = self.end_slot;
        Indexer {
            db,
            source,
            start_slot,
            end_slot,
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

pub struct Indexer {
    db: Database,
    source: LighthouseStore,
    start_slot: u64,
    end_slot: u64,
}

impl Indexer {
    pub fn run(&mut self) {
        self.source.run_index();
    }

    pub fn get_attestation_participation(&self, epoch: u64) -> Result<f64, IndexerError> {
        unimplemented!()
    }

    pub fn get_sync_committee_participation(&self, slot: u64) -> Result<f64, IndexerError> {
        unimplemented!()
    }
}
