mod lighthouse_store;

use eth2::types::eth_spec::MainnetEthSpec;
use eth2::types::{BeaconState, SignedBeaconBlock};

pub use lighthouse_store::LighthouseStore;

pub enum ChainSourceError {
    Missing,
}

pub trait ChainSource {
    fn get_block(&self, slot: u64) -> SignedBeaconBlock<MainnetEthSpec>;
    fn get_state(&self, slot: u64) -> BeaconState<MainnetEthSpec>;
}
