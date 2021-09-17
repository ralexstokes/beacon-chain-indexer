use super::ChainSource;
use beacon_chain::store::{KeyValueStore, LevelDB};
use eth2::types::eth_spec::MainnetEthSpec;
use eth2::types::{BeaconState, Hash256, SignedBeaconBlock};
use std::convert::TryInto;
use std::path::PathBuf;

const SPLIT_COL: &'static str = "bma";
const SPLIT_KEY: Hash256 = Hash256::repeat_byte(2);

pub struct LighthouseStore {
    cold_db: LevelDB<MainnetEthSpec>,
    hot_db: LevelDB<MainnetEthSpec>,
    split_slot: u64,
}

fn read_split_slot(db: &LevelDB<MainnetEthSpec>) -> u64 {
    let result = db
        .get_bytes(SPLIT_COL, &SPLIT_KEY.to_fixed_bytes())
        .expect("can read");
    let result = result.unwrap_or_else(|| vec![]);
    let slot_data = &result[0..8];
    u64::from_le_bytes(slot_data.try_into().expect("is correct"))
}

impl LighthouseStore {
    pub fn new<P: Into<PathBuf>>(beacon_data_dir_path: P) -> Self {
        let data_dir_path = beacon_data_dir_path.into();
        let mut cold_path = data_dir_path.clone();
        cold_path.push("beacon");
        cold_path.push("freezer_db");
        let cold_db = LevelDB::<MainnetEthSpec>::open(&cold_path).expect("can open DB");

        let mut hot_path = data_dir_path;
        hot_path.push("beacon");
        hot_path.push("chain_db");
        let hot_db = LevelDB::<MainnetEthSpec>::open(&hot_path).expect("can open DB");

        let split_slot = read_split_slot(&hot_db);

        Self {
            cold_db,
            hot_db,
            split_slot,
        }
    }
}

impl ChainSource for LighthouseStore {
    fn get_block(&self, slot: u64) -> SignedBeaconBlock<MainnetEthSpec> {
        unimplemented!()
    }

    fn get_state(&self, slot: u64) -> BeaconState<MainnetEthSpec> {
        unimplemented!()
    }
}
