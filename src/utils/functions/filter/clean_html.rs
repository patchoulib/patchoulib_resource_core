use scraper::node::Node;
use ego_tree::Tree;

/// This function will remove all the unnecessary tags from the HTML.
/// Only these tags will be kept:
/// - h1 - h6
/// - p
/// - a
/// - img
/// - ul, ol, li
/// - blockquote
/// - pre
/// - code
/// - table, thead, tbody, tfoot, tr, th, td
/// - ruby, rt, rp
/// - div, span
/// - header, footer, section, article, aside, nav, main
/// - body


static KEPT_TAGS: [&str; 35] = [
    "h1", "h2", "h3", "h4", "h5", "h6",
    "p",
    "a",
    "img",
    "ul", "ol", "li",
    "blockquote",
    "pre",
    "code",
    "table", "thead", "tbody", "tfoot", "tr", "th", "td",
    "ruby", "rt", "rp",
    "div", "span",
    "header", "footer", "section", "article", "aside", "nav", "main",
    "body",
];

fn judge(node: Node) -> bool {
    match node {
        Node::Element(element) => {
            let tag = element.name();
            KEPT_TAGS.contains(&tag)
        },
        Node::Text(_) => true,
        _ => false,
    }
}

fn clean_html(tree: Tree<Node>) -> Tree<Node> {
    let mut new_tree = Tree::new(Node::Document);
    for node in tree.nodes() {
        let mut new_root = new_tree.root_mut();
        match node {
            Node::Element(element) => {
                let tag = element.name();
                if KEPT_TAGS.contains(&tag) {
                    new_root.append(Node::Element(element));
                }
            },
            Node::Text(text) => {
                new_root.append(Node::Text(text));
            },
            _ => {},
        }
    }

    todo!()
}