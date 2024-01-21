use std::convert::TryInto;

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
    fn from_str(i: &str) -> Self {
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

    fn to_str(&self) -> &str {
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

pub trait HtmlSerialize {
    fn to_html_string(&self) -> &str;
}

pub type HtmlElement = impl HtmlSerialize + Sized;

pub struct NoAttributeHtmlTag {
    pub tag: HtmlTag,
    pub children: Vec<HtmlElement>,
}

impl HtmlSerialize for NoAttributeHtmlTag {
    fn to_html_string(&self) -> &str {
        let mut html_string = String::new();
        html_string.push_str("<");
        html_string.push_str(self.tag.to_str());
        html_string.push_str(">");
        for child in &self.children {
            html_string.push_str(child.to_html_string());
        }
        html_string.push_str("</");
        html_string.push_str(self.tag.to_str());
        html_string.push_str(">");
        html_string.as_str()
    }
}

pub struct ImageHtmlTag {
    // pub tag: HtmlTag::Img,
    pub src: String,
}

impl HtmlSerialize for ImageHtmlTag {
    fn to_html_string(&self) -> &str {
        format!(r#"<img src="{}" />"#, self.src).as_str()
    }
}

pub struct LinkHtmlTag {
    // pub tag: HtmlTag::A,
    pub href: String,
    pub children: Vec<HtmlElement>,
}

impl HtmlSerialize for LinkHtmlTag {
    fn to_html_string(&self) -> &str {
        let mut html_string = String::new();
        html_string.push_str(r#"<a href=""#);
        html_string.push_str(self.href.as_str());
        html_string.push_str(r#"">"#);
        for child in &self.children {
            html_string.push_str(child.to_html_string());
        }
        html_string.push_str(r#"</a>"#);
        html_string.as_str()
    }
}

pub struct NoHtmlTag {
    // pub tag: HtmlTag::String,
    pub text: String,
}

impl HtmlSerialize for NoHtmlTag {
    fn to_html_string(&self) -> &str {
        self.text.as_str()
    }
}

/// XML only allow 1 root tag.
/// to use more than 1 root tag, we need to wrap it with this tag
pub struct RootHelperTag {
    // pub tag: HtmlTag::Root,
    pub children: Vec<HtmlElement>,
}

impl HtmlSerialize for RootHelperTag {
    fn to_html_string(&self) -> &str {
        let mut html_string = String::new();
        for child in &self.children {
            html_string.push_str(child.to_html_string());
        }
        html_string.as_str()
    }
}