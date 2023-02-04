//! This binary generates a merkle tree for each [TipDistributionAccount]; they are derived
//! using a user provided [StakeMetaCollection] JSON file.

use {
    clap::Parser,
    log::*,
    serde::de::DeserializeOwned,
    solana_client::rpc_client::RpcClient,
    solana_tip_distributor::{GeneratedMerkleTreeCollection, StakeMetaCollection},
    std::{fs::File, io::BufReader, path::PathBuf},
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to JSON file containing the [StakeMetaCollection] object.
    #[clap(long, env)]
    stake_meta_coll_path: PathBuf,

    /// Path to JSON file to get populated with tree node data.
    #[clap(long, env)]
    out_path: PathBuf,

    #[clap(long, env)]
    rpc_url: String,
}

fn main() {
    env_logger::init();
    info!("amounts-diff...");

    let args: Args = Args::parse();

    let stake_meta_coll: StakeMetaCollection =
        read_json_from_file(&args.stake_meta_coll_path).unwrap();

    let _merkle_tree_coll = GeneratedMerkleTreeCollection::new_from_stake_meta_collection(
        stake_meta_coll,
        Some(RpcClient::new(args.rpc_url)),
    )
    .unwrap();
}

fn read_json_from_file<T>(path: &PathBuf) -> serde_json::Result<T>
where
    T: DeserializeOwned,
{
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader)
}
