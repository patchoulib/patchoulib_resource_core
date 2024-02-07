use sea_orm::{
    ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
    EnumIter, PrimaryKeyTrait,
};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "book_series")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub series_id: Uuid,

    #[sea_orm(index, unique)]
    pub main_title: String,

    #[sea_orm(index, unique)]
    pub japanese_title: String,

    #[sea_orm(column_type = "Json")]
    pub alias: Vec<String>,

    #[sea_orm(column_type = "Json")]
    pub authors: Vec<String>,

    #[sea_orm(column_type = "Json")]
    pub illustrator: Vec<String>,

    #[sea_orm(column_type = "Text")]
    pub description: String,

    #[sea_orm(column_type = "Json")]
    pub tags: Vec<String>,

    pub cover_from_item: Uuid,

    pub custom_properties: String, // json
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub type BookSeries = Model;
