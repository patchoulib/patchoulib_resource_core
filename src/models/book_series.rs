use uuid::Uuid;

struct Model {
    series_id: Uuid,
    main_title: String,
    japanese_title: String,
    other_titles: Vec<String>,

    authors: Vec<String>,

    illustrator: Vec<String>,

    description: String,

    tags: Vec<String>,

    resources: Vec<Uuid>,

    cover_from_item: Uuid,

    custom_properties: String,  // json
}