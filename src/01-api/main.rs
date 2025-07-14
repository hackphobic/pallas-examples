use blockfrost::{BlockFrostSettings, BlockfrostAPI, BlockfrostResult, Pagination};

pub mod 01_api;
use std::hash::Hash;
use std::fs;
use std::io::{Write, Read};
use blake2::{Blake2b, Blake2bVar};
use pallas::ledger::{addresses::ShelleyDelegationPart, traverse::ComputeHash};
use pallas::crypto::key::ed25519::{self, *};
use pallas::crypto::hash::Hasher;
use pallas::txbuilder::{Input, Output, BuiltTransaction, StagingTransaction};

fn build_api() -> BlockfrostResult<BlockfrostAPI> {
    let settings = BlockFrostSettings::new();
    let api = BlockfrostAPI::new("preprodtuBBTofEM0jqmUQF4NPwpUO2I9QkrHo6", settings);

    Ok(api)
}

#[tokio::main]
async fn main() {



    let api = build_api()?;
    let pagination = Pagination::default();
    println!("Fetching ...");
    
    // Health
    let root = api.root().await;
    let health = api.health().await;
    let health_clock = api.health_clock().await;
    let addresses_utxos = api.addresses_utxos(&address, pagination).await;
    println!("utxos: {:?}", addresses_utxos);

}