use std::iter;

pub use rand::{random, Rng, RngCore, thread_rng};
use rand::distributions::Alphanumeric;
pub use rand::distributions::uniform::{SampleRange, SampleUniform};
pub use uuid::Uuid;

/// 生成随机字符串（`0-9，a-z，A-Z`）
pub fn random_string_simple(len: usize) -> String {
    let mut rng = thread_rng();
    iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(len)
        .collect()
}

/// 随机生成在指定范围内的值
pub fn random_range<T, R>(range: R) -> T
    where T: SampleUniform,
          R: SampleRange<T>
{
    thread_rng().gen_range(range)
}

/// 生成带连字符的 UUID-v4 字符串（小写）
pub fn uuid_v4_hyphenated_lower_encode() -> String {
    Uuid::new_v4()
        .to_hyphenated()
        .encode_lower(&mut Uuid::encode_buffer())
        .into_string()
}

/// 生成无连字符的 UUID-v4 字符串（小写）
pub fn uuid_v4_simple_lower_encode() -> String {
    Uuid::new_v4()
        .to_simple()
        .encode_lower(&mut Uuid::encode_buffer())
        .into_string()
}