pub mod address;
pub mod keypair;
pub mod util;

use std::hash::Hash;
use std::fs;
use std::io::{Write, Read};
use blake2::{Blake2b, Blake2bVar};
use pallas::ledger::{
    addresses::{Network, ShelleyAddress, ShelleyDelegationPart, ShelleyPaymentPart}, 
    traverse::ComputeHash};
use pallas::crypto::key::ed25519::{self, *};
use pallas::crypto::hash::Hasher;
use pallas::txbuilder::{Input, Output, BuiltTransaction, StagingTransaction};
use pallas::ledger::primitives::NetworkId;
use address::print_current_address;
use keypair::generate_keypair;
use util::wallet_path;

// Loads a wallet from the keystore at "~/Home/.cardano/keystore".
// If the directory or file is missing, generates and stores a new keypair at that location.
#[tokio::main]
async fn main() {
    if let Ok(path) = wallet_path() {
        print_current_address(path)
    } else {
        generate_keypair(),
    }
}




        let parsed_key: [u8; 64] = key[..64].try_into().expect("Error: the provided key is shorter than 64 bytes");
        let xprv: SecretKeyExtended = SecretKeyExtended::from_bytes(parsed_key).expect("Error: parsing the key");
        let leaked: [u8; SecretKeyExtended::SIZE] = unsafe { SecretKeyExtended::leak_into_bytes(xprv.clone()) };
        let pubkey_hash = xprv.public_key().compute_hash();
        let mut hasher = Hasher::<224>::new();
        //hasher.input(xprv.public_key());
        let payment_part = ShelleyPaymentPart::Key(pubkey_hash);
        let delegation_part = ShelleyDelegationPart::Null;
        let network_part = Network::from(TESTNET_MAGIC);
        let address = ShelleyAddress::new(network_part, payment_part, delegation_part).to_bech32().expect("Error: Bech32");
        
        println!("Address: {:?}", address);
        println!("---------------------------------");
        println!();
        println!("Raw bytes: {:?}", leaked); //xprv.public_key().hash(&mut hasher));
        //let utf8 = String::from_utf8(leaked.to_vec()).unwrap();
        //let hex = leaked.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();
        println!("---------------------------------");
        println!("Hex encoded key: {:02X?}", leaked); 
        //let utf8 = leaked.iter().map(|&byte| String::from_utf8(byte)).collect::<String>();
        //print!("{:?}", hex);
/*

        let delegation_part = ShelleyDelegationPart(null);

        let input1 = Input::new(tx_hash, utxo_index);
        let output1 = Output::new(address, amount);
        output1.set_inline_datum();
        output1.set_inline_script();
        output1.set_datum_hash();

        let staging_tx = StagingTransaction::new();
        staging_tx.input(input);
        staging_tx.Output(output1);
        let built_tx = staging_tx.build_conway_raw();
*/



