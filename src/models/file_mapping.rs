use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "file_mappings")]
pub struct Model {
    #[sea_orm(index, unique)]
    pub virtual_path: String,

    #[sea_orm(primary_key)]
    pub hash: String,

    /// Must be not null.
    /// This column is used to garbage collect files that are not used.
    /// Foreign key to the items table.
    pub belongs_to_item: Uuid,

    pub file_extension: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub type FileMapping = Model;

impl Related<super::nav_point::Entity> for Entity {
    fn to() -> RelationDef {
        super::nav_point::Relation::FileMapping.def()
    }
}