pub const MIME_TYPE: &str = "application/epub+zip";
pub const CONTAINER_XML: &str = r#"
<?xml version="1.0"?>
<container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container">
    <rootfiles>
        <rootfile full-path="content.opf" media-type="application/oebps-package+xml"/>
    </rootfiles>
</container>
"#;