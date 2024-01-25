use sea_orm::{DeriveEntityModel, DeriveRelation, EnumIter, EntityTrait, DerivePrimaryKey, PrimaryKeyTrait, ActiveModelBehavior};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "file_mappings")]
pub struct Model {
    #[sea_orm(index, unique)]
    pub virtual_path: String,

    #[sea_orm(primary_key)]
    pub sha1: String,

    pub file_extension: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}