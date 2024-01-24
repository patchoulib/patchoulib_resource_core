use sea_orm::entity::prelude::*;
use sea_orm::{ColumnTrait, TryGetableFromJson};
use sea_orm::EntityTrait;
use sea_orm::PrimaryKeyTrait;
use sea_orm::ActiveModelBehavior;
use sea_orm::sea_query::{ArrayType, ValueType, ValueTypeErr};


use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "book_items")]
pub struct Model {
    // Database columns
    #[sea_orm(primary_key)]
    pub unique_id: Uuid,

    /// ## which book series this book item belongs to
    ///
    /// + index: true
    /// + not_null: true
    /// + foreign_key: true
    pub belongs_to: Uuid,

    /// ## the order of this book in the series
    pub order_in_series: u16,

    // Displayed columns
    /// ## the relative path of cover image
    pub cover_image: String,

    /// ## the name of the book item.
    ///
    /// For some books, the name is the same as the book series.
    pub item_name: String,

    #[sea_orm(column_type = "Json")]
    /// ## The pages' relative path in the epub.
    ///
    /// Stored as JSON in postgresql
    pub nav_points: NavPoints,

    pub status: BookItemStatus,

    /// When the book is being processed, this field is true.
    ///
    /// if the book is locked, it cannot be processed until the lock is released.
    pub processing_lock: bool,

    #[sea_orm(column_type = "Json")]
    /// ## The `content.opf` file in the epub. Stored as JSON in postgresql
    ///
    /// this pattern is used to generate the `content.opf` file in the epub.
    pub addition_info: Json,

    /// ## The path of the epub file
    /// if no epub file is created, this field is ""
    pub epub_path: String,

    /// ## The version of the epub file
    /// if no epub file is created, this field is 0.
    /// every time the epub is updated, this field will be increased by 1.
    ///
    /// When `epub_version` is less than `book_version`, the epub file should be updated.
    pub epub_version: i64,

    /// ## The version of the book files
    /// if no file is created, this field is 0.
    /// every time the book files are updated, this field will be increased by 1.
    pub book_version: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub type BookItem = Model;

/// ## The status of the book item
///
/// If `status` is `Initializing` or `Error`, the book will not display in the book list
#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "book_item_status")]
pub enum BookItemStatus {
    /// Before the first version of the epub is created.
    ///
    /// use this status when first uploading a book
    #[sea_orm(string_value = "initializing")]
    Initializing,

    /// The epub is ready to be downloaded.
    ///
    /// When the book updates, the status will be changed to Processing until
    /// the new version is ready.
    #[sea_orm(string_value = "ready")]
    Ready,

    /// Cannot create the epub for some reason.
    /// Or, just mark it as error
    #[sea_orm(string_value = "error")]
    Error,

    /// The first version of the epub was created,
    /// the new version is being created.
    ///
    /// In this status, epub can be download but it will be the old version
    #[sea_orm(string_value = "processing")]
    Processing,
}

#[derive(Serialize, Deserialize, Clone,  PartialEq, Debug)]
pub struct NavPoint {
    pub href: String,   // sha1
    pub label: String,
}


#[derive(Serialize, Deserialize, Clone,  PartialEq, Debug)]
pub struct NavPoints(pub Vec<NavPoint>);
impl Into<Value> for NavPoints {
    fn into(self) -> sea_orm::Value {
        let result = serde_json::to_value(self);
        match result {
            Ok(value) => Value::Json(Some(Box::from(value))),
            Err(_) => Value::Json(None),
        }
    }
}

impl TryGetableFromJson for NavPoints {}

impl ValueType for NavPoints {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::Json(Some(v)) => {
                let result = serde_json::from_value(*v);
                match result {
                    Ok(value) => Ok(value),
                    Err(e) => Err(ValueTypeErr{}),
                }
            }
            _ => Err(ValueTypeErr{}),
        }
    }

    fn type_name() -> String {
        "Json".to_string()
    }

    fn array_type() -> ArrayType {
        ArrayType::Json
    }

    fn column_type() -> ColumnType {
        ColumnType::Json
    }
}



/// ## The `content.opf` file in the epub
/// standard: epub3
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PackageFormat {
    pub title: Option<String>,
    pub creator: Option<String>,
    pub language: Option<String>,
    pub publisher: Option<String>,
    pub rights: Option<String>,
    pub description: Option<String>,
    pub date: Option<String>,
    pub identifier: Option<String>,
    pub subject: Option<String>,
    pub source: Option<String>,
    pub type_: Option<String>,
    pub format: Option<String>,
}
