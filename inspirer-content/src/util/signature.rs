use crate::error::{Error, InspirerContentResult};
use base64ct::LineEnding;
use ring::{
    rand,
    signature::{self, KeyPair},
};

use super::hash::sha256;

pub fn generate_pkcs8_keypair() -> InspirerContentResult<Pkcs8KeyPair> {
    let rng = rand::SystemRandom::new();

    let pcks8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng)?;
    let key_pair = signature::Ed25519KeyPair::from_pkcs8(pcks8_bytes.as_ref())
        .or(Err(Error::RingKeyPairFormatError))?;

    Ok(Pkcs8KeyPair {
        private_key: pcks8_bytes.as_ref().to_vec(),
        public_key: key_pair.public_key().as_ref().to_vec(),
    })
}

pub fn private_key_to_pem(private_key: &[u8]) -> InspirerContentResult<String> {
    let pem = der::Document::try_from(private_key).or(Err(Error::RingKeyPairFormatError))?;
    pem.to_pem("PRIVATE KEY", LineEnding::LF)
        .or(Err(Error::RingKeyPairFormatError))
}

pub struct Pkcs8KeyPair {
    pub private_key: Vec<u8>,
    pub public_key: Vec<u8>,
}

impl Pkcs8KeyPair {
    pub fn private_key_bytes(&self) -> &[u8] {
        &self.private_key
    }

    pub fn public_key_bytes(&self) -> &[u8] {
        &self.public_key
    }

    pub fn public_key_fingerprint(&self) -> Vec<u8> {
        sha256(self.public_key_bytes())
    }
}

pub fn unparsed_public_key(public_key: &[u8]) -> signature::UnparsedPublicKey<&[u8]> {
    signature::UnparsedPublicKey::new(&signature::ED25519, public_key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64ct::LineEnding;
    use der::Document;
    use ring::signature::{self, KeyPair};

    #[test]
    fn test_pkcs8() {
        let raw_key_pair = generate_pkcs8_keypair().unwrap();

        const MESSAGE: &[u8] = b"hello, world";
        let key_pair =
            signature::Ed25519KeyPair::from_pkcs8(raw_key_pair.private_key_bytes()).unwrap();
        let sig = key_pair.sign(MESSAGE);

        let doc = Document::try_from(raw_key_pair.private_key_bytes()).unwrap();
        let res = doc.to_pem("PRIVATE KEY", LineEnding::LF).unwrap();

        println!("{res}");

        let (_, doc) = Document::from_pem(&res).unwrap();
        let key_pair = signature::Ed25519KeyPair::from_pkcs8(doc.as_bytes()).unwrap();

        let pub_key =
            signature::UnparsedPublicKey::new(&signature::ED25519, key_pair.public_key().as_ref());

        assert!(pub_key.verify(MESSAGE, sig.as_ref()).is_ok())
    }
}
