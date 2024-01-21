use std::collections::HashMap;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::utils::epub_page_object::html::HtmlElement;

pub enum HtmlTag {
    H1, H2, H3, H4, H5, H6,
    P, Div, Span,
    Img, Svg,
    Ul, Ol, Li,
    Br,
    Strong, Em, B, I,
    A,
    Pre, Code,
    Ruby, Rt, Rp,
    String, // no tag
    NotSupported,   // not supported tag
    Root,   // root tag
}

impl HtmlTag {
    pub fn from_str(i: &str) -> Self {
        match i {
            "h1" => HtmlTag::H1,
            "h2" => HtmlTag::H2,
            "h3" => HtmlTag::H3,
            "h4" => HtmlTag::H4,
            "h5" => HtmlTag::H5,
            "h6" => HtmlTag::H6,
            "p" => HtmlTag::P,
            "div" => HtmlTag::Div,
            "span" => HtmlTag::Span,
            "img" => HtmlTag::Img,
            "svg" => HtmlTag::Svg,
            "ul" => HtmlTag::Ul,
            "ol" => HtmlTag::Ol,
            "li" => HtmlTag::Li,
            "br" => HtmlTag::Br,
            "strong" => HtmlTag::Strong,
            "em" => HtmlTag::Em,
            "b" => HtmlTag::B,
            "i" => HtmlTag::I,
            "a" => HtmlTag::A,
            "pre" => HtmlTag::Pre,
            "code" => HtmlTag::Code,
            "ruby" => HtmlTag::Ruby,
            "rt" => HtmlTag::Rt,
            "rp" => HtmlTag::Rp,
            "" => HtmlTag::String,
            " " => HtmlTag::String,
            _ => HtmlTag::NotSupported,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            HtmlTag::H1 => "h1",
            HtmlTag::H2 => "h2",
            HtmlTag::H3 => "h3",
            HtmlTag::H4 => "h4",
            HtmlTag::H5 => "h5",
            HtmlTag::H6 => "h6",
            HtmlTag::P => "p",
            HtmlTag::Div => "div",
            HtmlTag::Span => "span",
            HtmlTag::Img => "img",
            HtmlTag::Svg => "svg",
            HtmlTag::Ul => "ul",
            HtmlTag::Ol => "ol",
            HtmlTag::Li => "li",
            HtmlTag::Br => "br",
            HtmlTag::Strong => "strong",
            HtmlTag::Em => "em",
            HtmlTag::B => "b",
            HtmlTag::I => "i",
            HtmlTag::A => "a",
            HtmlTag::Pre => "pre",
            HtmlTag::Code => "code",
            HtmlTag::Ruby => "ruby",
            HtmlTag::Rt => "rt",
            HtmlTag::Rp => "rp",
            HtmlTag::Root => "",
            HtmlTag::String => "",
            HtmlTag::NotSupported => "",
        }
    }
}

pub struct XmlAttribute {
    pub name: String,
    pub value: String,
}

pub struct NoAttributeHtmlTag {
    pub tag: HtmlTag,
    pub children: Vec<Html>,
}

pub struct ImageHtmlTag {
    // pub tag: HtmlTag::Img,
    pub src: String,
}

pub struct LinkHtmlTag {
    // pub tag: HtmlTag::A,
    pub href: String,
    pub children: Vec<Html>,
}

pub struct NoHtmlTag {
    // pub tag: HtmlTag::String,
    pub text: String,
}

pub struct RootTag {
    // pub tag: HtmlTag::Root,
    pub children: Vec<Html>,
}

pub enum Html {
    NoAttributeHtmlTag(NoAttributeHtmlTag),
    ImageHtmlTag(ImageHtmlTag),
    LinkHtmlTag(LinkHtmlTag),
    NoHtmlTag(NoHtmlTag),
    RootTag(RootTag),
}

impl Serialize for Html {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_struct("Html", 3)?;

        match self {
            Html::NoAttributeHtmlTag(tag) => {
                state.serialize_field("tagName", &tag.tag.to_str())?;
                state.serialize_field("children", &tag.children)?;
            },
            Html::ImageHtmlTag(tag) => {
                state.serialize_field("tagName", "img")?;
                let mut attributes = HashMap::new();
                attributes.insert("src".to_string(), tag.src.clone());
                state.serialize_field("attributes", &attributes)?;
            },
            Html::LinkHtmlTag(tag) => {
                state.serialize_field("tagName", "a")?;
                let mut attributes = HashMap::new();
                attributes.insert("href".to_string(), tag.href.clone());
                state.serialize_field("attributes", &attributes)?;
                state.serialize_field("children", &tag.children)?;
            },
            Html::NoHtmlTag(tag) => {
                // NoHtmlTag is serialized as a string, not an object
                return tag.text.serialize(serializer);
            },
            Html::RootTag(tag) => {
                state.serialize_field("tagName", "root")?;
                state.serialize_field("children", &tag.children)?;
            },
        }

        state.end()
    }
}