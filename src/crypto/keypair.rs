use pallas::crypto::key::ed25519::{SecretKeyExtended, TryFromSecretKeyExtendedError};
use rand_core::OsRng;


pub fn xprv_from_rng() -> Result<SecretKeyExtended, TryFromSecretKeyExtendedError> {
    let mut rng = OsRng;
    let xprv = SecretKeyExtended::new(&mut rng);
    Ok(xprv)
}