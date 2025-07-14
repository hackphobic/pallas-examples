use pallas::crypto::key::ed25519::{SecretKeyExtended, TryFromSecretKeyExtendedError};
use rand_core::OsRng;
use util::wallet_path;


pub fn generate_keypair() -> Result<SecretKeyExtended, TryFromSecretKeyExtendedError> {
    let mut rng = OsRng;
    let xprv = SecretKeyExtended::new(&mut rng);

    fs::create_dir_all(wallet_path())?;

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