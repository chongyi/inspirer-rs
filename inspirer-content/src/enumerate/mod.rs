pub mod content {
    use sea_orm::{DeriveActiveEnum, EnumIter};
    use serde::Serialize;
    use strum::{EnumString, EnumVariantNames};

    use crate::model::content::ContentEntity;

    #[derive(Debug, Clone, Copy, PartialEq, EnumIter, DeriveActiveEnum, Serialize, EnumString, EnumVariantNames)]
    #[sea_orm(rs_type = "u32", db_type = "Unsigned")]
    #[strum(serialize_all = "kebab-case")]
    #[repr(u32)]
    pub enum ContentType {
        #[sea_orm(num_value = 1)]
        Post = 1,
        #[sea_orm(num_value = 2)]
        Page = 2,
    }

    impl From<ContentEntity> for ContentType {
        fn from(entity: ContentEntity) -> Self {
            match entity {
                ContentEntity::Post(_) => ContentType::Post,
                ContentEntity::Page(_) => ContentType::Page,
            }
        }
    }

    impl From<&ContentEntity> for ContentType {
        fn from(entity: &ContentEntity) -> Self {
            match entity {
                ContentEntity::Post(_) => ContentType::Post,
                ContentEntity::Page(_) => ContentType::Page,
            }
        }
    }
}
