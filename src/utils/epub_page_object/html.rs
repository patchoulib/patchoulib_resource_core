pub trait HtmlSerialize {
    fn to_html_string(&self) -> &str;
}

pub type HtmlElement = impl HtmlSerialize + Sized;

