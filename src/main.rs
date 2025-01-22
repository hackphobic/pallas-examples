pub mod crypto;
use std::hash::Hash;
use blake2::{Blake2b, Blake2bVar};
use pallas::ledger::{addresses, traverse::ComputeHash};
use pallas::crypto::key::ed25519::{self, *};
use pallas::crypto::hash::Hasher;

use crypto::{
    //address::{xprv_from_phrase}, 
    keypair::{xprv_from_rng}
};

fn main() {
    
    let xprv: SecretKeyExtended = xprv_from_rng().expect("Error");
    let leaked: [u8; SecretKeyExtended::SIZE] = unsafe { SecretKeyExtended::leak_into_bytes(xprv) };
    //let pubkey_hash = xprv.public_key().compute_hash();
    let mut hasher = Hasher::<224>::new();
    //hasher.input(xprv.public_key());
    //let payment_key = ShelleyPaymentPart::Key(Hash::<28>::from(xprv.public_key().compute_hash()));
    print!("{:?}", leaked); //xprv.public_key().hash(&mut hasher));

    let utf8 = leaked.iter().map(|&c| c as char).collect::<String>();
    print!("{:?}", utf8);
}