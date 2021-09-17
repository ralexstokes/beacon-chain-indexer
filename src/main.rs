use beacon_chain_indexer::{Builder, ChainSource, Indexer};
use clap::Clap;
use std::collections::HashMap;

const PRATER_ALTAIR_FORK_EPOCH: u64 = 36660;
const SLOTS_PER_EPOCH: u64 = 32;

#[derive(Clap)]
struct Config {
    #[clap(long)]
    lighthouse_db_path: String,

    #[clap(long, default_value = "beacon-chain-index")]
    index_db_path: String,

    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Index,
    Analyze,
}

fn main() {
    let config = Config::parse();

    let prater_altair_fork_slot = SLOTS_PER_EPOCH * PRATER_ALTAIR_FORK_EPOCH;
    let mut indexer = Builder::new()
        .with_index_db(config.index_db_path)
        .from_lighthouse(config.lighthouse_db_path)
        .with_start_slot(prater_altair_fork_slot)
        .build();

    match config.subcmd {
        SubCommand::Index => {
            indexer.run();
        }
        SubCommand::Analyze => {
            run_prater_sync_committee_analysis(indexer);
        }
    }
}

#[derive(Debug)]
struct EpochResult {
    attestation_participation: f64,
}

fn run_prater_sync_committee_analysis<C: ChainSource>(indexer: Indexer<C>) {
    let fork_epoch = PRATER_ALTAIR_FORK_EPOCH;
    let head_epoch = 39900;
    let mut epoch_results = HashMap::new();
    let mut slot_results = HashMap::new();
    for epoch in fork_epoch..=head_epoch {
        for slot in epoch * SLOTS_PER_EPOCH..(epoch + 1) * SLOTS_PER_EPOCH {
            let sync_committee_participation = indexer
                .get_sync_committee_participation(slot)
                .expect("slot processed");
            slot_results.insert(slot, sync_committee_participation);
        }
        let attestation_participation = indexer
            .get_attestation_participation(epoch)
            .expect("epoch processed");
        epoch_results.insert(
            epoch,
            EpochResult {
                attestation_participation,
            },
        );
    }
    dbg!(epoch_results);
    dbg!(slot_results);
}
