use sea_orm::{
    ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, EntityTrait,
    EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait,
};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "file_mappings")]
pub struct Model {
    #[sea_orm(index, unique)]
    pub virtual_path: String,

    #[sea_orm(primary_key)]
    pub hash: String,

    #[sea_orm(index)]
    /// Must be not null.
    /// This column is used to garbage collect files that are not used.
    /// Foreign key to the items table.
    pub belongs_to_item: Uuid,

    pub file_extension: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    BookItem,
}

impl ActiveModelBehavior for ActiveModel {}

pub type FileMapping = Model;

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::BookItem => Entity::belongs_to(super::book_item::Entity)
                .from(Column::BelongsToItem)
                .to(super::book_item::Column::UniqueId)
                .into(),
        }
    }
}

impl Related<super::book_item::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BookItem.def()
    }
}

impl Related<super::nav_point::Entity> for Entity {
    fn to() -> RelationDef {
        super::nav_point::Relation::FileMapping.def()
    }
}
