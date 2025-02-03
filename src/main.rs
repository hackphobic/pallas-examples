pub mod wallet;
use std::hash::Hash;
use std::fs;
use std::io::{Write, Read};
use blake2::{Blake2b, Blake2bVar};
use pallas::ledger::{addresses::ShelleyDelegationPart, traverse::ComputeHash};
use pallas::crypto::key::ed25519::{self, *};
use pallas::crypto::hash::Hasher;
use pallas::txbuilder::{Input, Output, BuiltTransaction, StagingTransaction};

use wallet::{
    //address::{xprv_from_phrase},
    keypair::{xprv_from_rng}
};

#[tokio::main]
async fn main() {
    if let Ok(key) = fs::read("C:\\Users\\is_st\\key.txt"){
        let parsed_key: [u8; 64] = key[..64].try_into().expect("Error: the provided key is shorter than 64 bytes");
        let xprv: SecretKeyExtended = SecretKeyExtended::from_bytes(parsed_key).expect("Error: parsing the key");
        let leaked: [u8; SecretKeyExtended::SIZE] = unsafe { SecretKeyExtended::leak_into_bytes(xprv) };
        let pubkey_hash = xprv.public_key().compute_hash();
        let mut hasher = Hasher::<224>::new();
        //hasher.input(xprv.public_key());
        let payment_part = ShelleyPaymentPart::Key(pubkey_hash);
        let delegation_part = ShelleyDelegationPart::Null;
        let network_part = Network::Testnet;
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
        let delegation_part = ShelleyDelegationPart(null);

        let inputs = Input::new(tx_hash, utxo_index);
        let output1 = Output::new(address, amount);
        output1.set_inline_datum();
        output1.set_inline_script();
        output1.set_datum_hash();





    } else {
        let xprv: SecretKeyExtended = xprv_from_rng().expect("Error");
        let leaked: [u8; SecretKeyExtended::SIZE] = unsafe { SecretKeyExtended::leak_into_bytes(xprv)};
        let write = fs::write("C:\\Users\\is_st\\key.txt", leaked);
        //let pubkey_hash = xprv.public_key().compute_hash();
        let mut hasher = Hasher::<224>::new();
        //hasher.input(xprv.public_key());
        //let payment_key = ShelleyPaymentPart::Key(Hash::<28>::from(xprv.public_key().compute_hash()));
        println!("Raw bytes: {:?}", leaked); //xprv.public_key().hash(&mut hasher)).;
        //let utf8 = String::from_utf8(leaked.to_vec()).unwrap();
        //let hex = leaked.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();
        println!();
        println!("Hex encoded key: {:02X?}", leaked);
        //let utf8 = leaked.iter().map(|&byte| String::from_utf8(byte)).collect::<String>();
        //print!("{:?}", hex);
        println!("writing key to file: {:?}", write);
    }
}