use uuid::Uuid;

use crate::error::InspirerContentResult;

#[async_trait::async_trait]
pub trait ContentDao {
    async fn create_content(&self) -> InspirerContentResult<Uuid>;
}