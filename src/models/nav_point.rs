use sea_orm::{DeriveEntityModel, EnumIter, DerivePrimaryKey, PrimaryKeyTrait, ActiveModelBehavior, EntityTrait, Related, RelationDef, RelationTrait};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "nav_points")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,

    /// Foreign key to the items table
    pub belongs_to_item: Uuid,

    /// Displayed name. For example, "Chapter 1"
    pub label: String,

    /// The order of the nav point in the epub
    pub order: i32,

    /// The hash of the page in the epub.
    /// Foreign key to the file_mappings table
    pub href: String,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    BookItem,
    FileMapping
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::BookItem => Entity::belongs_to(super::book_item::Entity)
                .from(Column::BelongsToItem)
                .to(super::book_item::Column::UniqueId)
                .into(),
            Self::FileMapping => Entity::has_one(super::file_mapping::Entity)
                .from(Column::Href)
                .to(super::file_mapping::Column::Hash)
                .into(),
        }
    }
}

impl Related<super::book_item::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BookItem.def()
    }
}
