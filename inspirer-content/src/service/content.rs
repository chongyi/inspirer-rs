#[async_trait::async_trait]
pub trait ContentService {
    async fn create_content();
}