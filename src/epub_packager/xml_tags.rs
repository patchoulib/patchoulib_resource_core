use crate::epub_packager::file_collect::{MediaType, Resource, ResourceList};
use crate::models::{BookItem, BookSeries, PackageFormat};

const XML_HEADER: &str = r#"<?xml version="1.0" encoding="UTF-8"?>"#;
const PACKAGE_TAG: &str = r#"<package xmlns="https://www.idpf.org/2007/opf" version="3.0" unique-identifier="uuid_id">"#;
const METADATA_TAG: &str = r#"<metadata xmlns:dc="https://purl.org/dc/elements/1.1/">"#;

const LANGUALGE_TAG: &str = r#"<dc:language>zh</dc:language>"#;

// tool functions used to generate xml tags
fn end_tag(tag_name: &str) -> String {
    format!("</{}>", tag_name)
}
fn start_tag(tag_name: &str) -> String {
    format!("<{}>", tag_name)
}
fn start_tag_with_attr(tag_name: &str, attr: &str) -> String {
    format!("<{} {}>", tag_name, attr)
}
fn self_closing_tag_with_attr(tag_name: &str, attr: &str) -> String {
    format!("<{} {} />", tag_name, attr)
}
fn tag_with_children(tag_name: &str, children: &str) -> String {
    format!("{}{}{}", start_tag(tag_name), children, end_tag(tag_name))
}

// mapping MediaType to String
impl TryInto<String> for MediaType {
    type Error = &'static str;

    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            MediaType::Image(ext) => Ok(format!("image/{}", ext)),
            MediaType::Svg => Ok("image/svg+xml".to_string()),
            MediaType::Xml => Ok("application/xhtml+xml".to_string()),
        }
    }
}

// generate manifest
fn manifest_item(resource: &Resource) -> String {
    let Resource { href, media_type, .. } = resource;
    let id = href.clone();
    let folder = match media_type {
        MediaType::Image(_) | MediaType::Svg => "OEBPS/Images",
        MediaType::Xml => "OEBPS/Text",
    };
    let media_type: String = media_type.clone().try_into().unwrap();
    let attr = format!(
        "id=\"{}\" href=\"{}\" media-type=\"{}\"",
        id,
        format!("{}/{}", folder, href),
        media_type
    );
    self_closing_tag_with_attr("item", &attr)
}
pub fn manifest(resources: &ResourceList) -> String {
    let items: Vec<String> = resources.iter().map(manifest_item).collect();
    tag_with_children("manifest", &items.join(""))
}

// generate metadata
fn dc_language() -> String {
    LANGUALGE_TAG.to_string()
}
fn dc_title(title: &str) -> String {
    tag_with_children("dc:title", title)
}
fn authors_tags(authors: Vec<String>) -> String {
    authors.iter()
        .map(|author| format!(r#"<dc:creator opf:role="aut">{}</dc:creator>"#, author))
        .collect::<Vec<String>>()
        .join("")
}
fn illustrator_tags(illustrators: Vec<String>) -> String {
    illustrators.iter()
        .map(|illustrator| format!(r#"<dc:creator opf:role="ill">{}</dc:creator>"#, illustrator))
        .collect::<Vec<String>>()
        .join("")
}
fn dc_description(description: &str) -> String {
    tag_with_children("dc:description", description)
}
fn package_format(additional_info: PackageFormat) -> String {
let PackageFormat {
        creator,
        publisher,
        rights,
        description,
        date,
        identifier,
        subject,
        source,
        type_,
        format,
    } = additional_info;
    let creator = creator.map(|c| tag_with_children("dc:creator", c.as_str()));
    let publisher = publisher.map(|p| tag_with_children("dc:publisher", p.as_str()));
    let rights = rights.map(|r| tag_with_children("dc:rights", r.as_str()));
    let description = description.map(|d| tag_with_children("dc:description", d.as_str()));
    let date = date.map(|d| tag_with_children("dc:date", d.as_str()));
    let identifier = identifier.map(|i| tag_with_children("dc:identifier", i.as_str()));
    let subject = subject.map(|s| tag_with_children("dc:subject", s.as_str()));
    let source = source.map(|s| tag_with_children("dc:source", s.as_str()));
    let type_ = type_.map(|t| tag_with_children("dc:type", t.as_str()));
    let format = format.map(|f| tag_with_children("dc:format", f.as_str()));
    let tags: Vec<String> = vec![
        creator,
        publisher,
        rights,
        description,
        date,
        identifier,
        subject,
        source,
        type_,
        format,
    ]
    .into_iter()
    .filter_map(|x| x)
    .collect();
    tag_with_children("metadata", &tags.join(""))
}
pub fn metadata(book_item: BookItem, book_series: BookSeries) -> String {
    let BookSeries {
        authors,
        illustrator,
        description,
        ..
    } = book_series;
    let BookItem {
        item_name,
        addition_info,
        ..
    } = book_item;
    let title = dc_title(item_name.as_str());
    let authors = authors_tags(authors);
    let illustrators = illustrator_tags(illustrator);
    let description = dc_description(description.as_str());
    let additional_info = addition_info.as_str();
    let package_format = match additional_info {
        Some(info) => {
            let info: PackageFormat = serde_json::from_str(info).unwrap();
            package_format(info)
        }
        None => "".to_string(),
    };
    let tags: Vec<String> = vec![dc_language(), title, authors, illustrators, description, package_format];
    tag_with_children("metadata", &tags.join(""))
}