use std::io;
use pallas::crypto::key::ed25519::{SecretKeyExtended, TryFromSecretKeyExtendedError};
use bip32::{Mnemonic, Language, ExtendedPrivateKey};
use ed25519_dalek::{SecretKey, SigningKey};
use bip32::{XPrv, DerivationPath};

pub fn xprv_from_phrase(seed_phrase: &str) -> Result<ExtendedPrivateKey<SigningKey>, io::Error> {
    let phrase = Mnemonic::new(seed_phrase, Language::English).expect("Invalid phrase");
    let seed = phrase.to_seed("");
    let xprv = XPrv::new(&seed).expect("Invalid seed");

    Ok(xprv)
    //let xprv: ExtendedPrivateKey<ecdsa::signing::SigningKey<Ed25519>> = ExtendedPrivateKey::new(seed); //from_bytes(&seed.as_bytes()[..32]).expect("Invalid secret key length");
    //let xprv = SecretKeyExtended::from_bytes(*seed.as_bytes()).expect("Invalid seed");
    
    /*.map_err(|e| match e {
        TryFromSecretKeyExtendedError::InvalidSeed => io::Error::new(io::ErrorKind::InvalidInput, "Invalid seed"),
        TryFromSecretKeyExtendedError::InvalidKey => io::Error::new(io::ErrorKind::InvalidInput, "Invalid key"),
    })?; */
}

pub fn keypair_from_phrase(seed_phrase: &str) -> Result<(XPrv, XPrv), io::Error> {
    let mnemonic = Mnemonic::new(seed_phrase, Language::English).expect("Invalid phrase");
    let seed = mnemonic.to_seed("");
    let xprv = XPrv::new(&seed).expect("Invalid seed");

    // Derive a child keypair from the master key
    let dp = DerivationPath::from_str("m/44'/60'/0'/0/0").expect("Invalid derivation path");
    let derivation_path = DerivationPath::from_str("m/44'/60'/0'/0/0").expect("Invalid derivation path");
    let child_xprv = xprv.derive(&derivation_path).expect("Failed to derive child key");

    Ok((xprv, child_xprv))
}
/* 
pub fn generate_address(keypair: &KeyPair) -> Result<(), io::Error> {
    let xpub = XPub::from_bytes(&keypair.public().to_bytes())?;

    // Generate Base Address
    let base_address = BaseAddress::new(0, &xpub, &xpub);
    println!("Base Address: {}", base_address.to_bech32()?);

    // Generate Enterprise Address
    let enterprise_address = EnterpriseAddress::new(0, &xpub);
    println!("Enterprise Address: {}", enterprise_address.to_bech32()?);

    // Generate Pointer Address
    let pointer_address = PointerAddress::new(0, &xpub, 1, 2, 3);
    println!("Pointer Address: {}", pointer_address.to_bech32()?);

    // Generate Reward Address
    let reward_address = RewardAddress::new(0, &xpub);
    println!("Reward Address: {}", reward_address.to_bech32()?);

    Ok(())
} */

