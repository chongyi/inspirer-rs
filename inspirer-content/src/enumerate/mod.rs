pub mod content {
    use sea_orm::{DeriveActiveEnum, EnumIter};

    use crate::model::content::ContentEntity;

    #[derive(Debug, Clone, Copy, PartialEq, EnumIter, DeriveActiveEnum)]
    #[sea_orm(rs_type = "u32", db_type = "Unsigned")]
    pub enum ContentType {
        #[sea_orm(num_value = 1)]
        Post,
    }

    impl From<ContentEntity> for ContentType {
        fn from(entity: ContentEntity) -> Self {
            match entity {
                ContentEntity::Post(_) => ContentType::Post,
            }
        }
    }

    impl From<&ContentEntity> for ContentType {
        fn from(entity: &ContentEntity) -> Self {
            match entity {
                ContentEntity::Post(_) => ContentType::Post,
            }
        }
    }
}
