use std::io;
use pallas::crypto::key::{Seed, KeyPair};

fn generate_keypair_from_seed(seed_phrase: &str) -> Result<KeyPair, io::Error> {
    let seed = Seed::from_phrase(seed_phrase)?;
    let keypair = KeyPair::from_seed(&seed)?;
    Ok(keypair)
}