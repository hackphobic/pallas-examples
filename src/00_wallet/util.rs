use std::path::PathBuf;
use anyhow::Result;

const WALLET_DIR: &str = ".cardano";
const WALLET_KEYSTORE_FILENAME: &str = "keystore";

pub fn wallet_path() -> Result<PathBuf> {
    match std::env::var_os("WALLET") {
        Some(wallet) => Ok(wallet.into()),
        None => match dirs::home_dir() {
            Some(v) => Ok(v.join(WALLET_DIR).join(WALLET_KEYSTORE_FILENAME)),
            None => None,
        },
    }
}