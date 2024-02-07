pub struct Resource {
    pub href: String,
    pub media_type: MediaType,
    pub content: ResourceContent,
}

#[derive(Debug, Clone)]
pub enum MediaType {
    Image(String),
    Svg,
    Xml,
}

#[derive(Debug, Clone)]
pub enum ResourceContent {
    Image(Vec<u8>),
    Xml(String),
}

pub type ResourceList = Vec<Resource>;