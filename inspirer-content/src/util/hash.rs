use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use sha2::{Digest, Sha256};

use crate::error::InspirerContentResult;

pub fn sha256(msg: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();

    hasher.update(msg);
    hasher.finalize().as_slice().to_vec()
}

pub fn password_hash(password: &str) -> InspirerContentResult<String> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_ref(), &salt)?.to_string();

    Ok(password_hash)
}

pub fn verify_password(password: &str, hash: &str) -> InspirerContentResult<bool> {
    let parsed_hash = PasswordHash::new(hash)?;

    Ok(Argon2::default()
        .verify_password(password.as_ref(), &parsed_hash)
        .is_ok())
}
