use sha2::{Sha256, Digest};

pub fn sha256(msg: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();

    hasher.update(msg);
    hasher.finalize().as_slice().to_vec()
}