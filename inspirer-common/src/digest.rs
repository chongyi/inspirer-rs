use digest::Digest as RustDigest;
pub use md5::Md5;
pub use sha1::Sha1;
pub use sha2::{Sha224, Sha256, Sha384, Sha512};
use std::marker::PhantomData;
use generic_array::{GenericArray, ArrayLength};

pub struct Digest<T>
{
    _phantom: PhantomData<T>
}

impl<T> Digest<T>
    where T: RustDigest,
          <T as RustDigest>::OutputSize: ArrayLength<u8>,
{
    pub fn digest<B: AsRef<[u8]>>(data: B) -> GenericArray<u8, <T as RustDigest>::OutputSize>
    {
        let mut hasher = T::new();
        hasher.input(data);
        hasher.result()
    }
}

pub fn md5<B: AsRef<[u8]>>(data: B) -> String {
    format!("{:x}", Digest::<Md5>::digest(data))
}

pub fn sha1<B: AsRef<[u8]>>(data: B) -> String {
    format!("{:x}", Digest::<Sha1>::digest(data))
}

pub fn sha256<B: AsRef<[u8]>>(data: B) -> String {
    format!("{:x}", Digest::<Sha256>::digest(data))
}