use crate::{model::user::NewUser, error::InspirerContentResult, manager::Manager, util::signature::generate_pkcs8_keypair};

#[async_trait::async_trait]
pub trait UserService {
    async fn create_user_simple(&self, new_user: NewUser) -> InspirerContentResult<()>;
}

#[async_trait::async_trait]
impl UserService for Manager {
    async fn create_user_simple(&self, new_user: NewUser) -> InspirerContentResult<()> {
        // 生成私钥公钥
        let key_pair = generate_pkcs8_keypair()?;

        // todo 逻辑待完成

        Ok(())
    }
}