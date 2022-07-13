use bs62::num_traits::ToPrimitive;
use chrono::Utc;
use mac_address::get_mac_address;
use uuid::v1::{Context, Timestamp};
use once_cell::sync::Lazy;
pub use uuid::Uuid;

use crate::error::{Error, InspirerContentResult};

static MAC_ADDRESS: Lazy<[u8; 6]> = Lazy::new(|| {
    get_mac_address()
    .expect("获取 MAC 地址信息失败")
    .expect("无法获取到有效的 MAC 地址")
    .bytes()
});

static CLOCK_SEQUENCE: Lazy<Context> = Lazy::new(|| {
    Context::new(0)
});

/// 生成 UUID v1
pub fn generate_v1_uuid() -> Uuid {
    let now = Utc::now();
    let ctx: &Context = &CLOCK_SEQUENCE;

    Uuid::new_v1(
        Timestamp::from_unix(
            ctx,
            now.timestamp() as u64,
            now.timestamp_subsec_nanos() as u32,
        ),
        &MAC_ADDRESS,
    )
}

/// 生成 UUID v4
pub fn generate_v4_uuid() -> Uuid {
    Uuid::new_v4()
}

/// 对 UUID 进行 62 进制转换
pub fn uuid_to_base62(uuid: Uuid) -> String {
    bs62::encode_num(&uuid.as_u128())
}

/// 转换 62 进制文本为 UUID，要求数值必须为 128 位整型
pub fn base62_to_uuid(num_str: &str) -> InspirerContentResult<Uuid> {
    bs62::decode_num(num_str)
        .and_then(|num| Ok(Uuid::from_u128(num.to_u128().ok_or(Error::ConvertIdError)?)))
        .or(Err(Error::ConvertIdError))
}
