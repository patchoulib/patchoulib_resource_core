use sea_orm::{
    ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
    EnumIter, PrimaryKeyTrait,
};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "book_series")]
pub struct Model {
    #[sea_orm(primary_key)]
    series_id: Uuid,

    #[sea_orm(index, unique)]
    main_title: String,

    #[sea_orm(index, unique)]
    japanese_title: String,

    #[sea_orm(column_type = "Json")]
    alias: Vec<String>,

    #[sea_orm(column_type = "Json")]
    authors: Vec<String>,

    #[sea_orm(column_type = "Json")]
    illustrator: Vec<String>,

    #[sea_orm(column_type = "Text")]
    description: String,

    #[sea_orm(column_type = "Json")]
    tags: Vec<String>,

    #[sea_orm(column_type = "Json")]
    resources: Vec<Uuid>,

    cover_from_item: Uuid,

    custom_properties: String, // json
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub type BookSeries = Model;
