use std::collections::HashMap;

struct Node {
    // Vec "vector" is a growable array data type
    children: Vec<Node>,
    node_type: NodeType,
}

/* The DOM has several node types, but we'll simplify and say
that a node is either an Element or Text.

For a complete list of nodeTypes: https://dom.spec.whatwg.org/#dom-node-nodetype */
enum NodeType {
    Text(String),
    Element(ElementData),
}

/* An element includes a tag name and unlimited number of attributes,
which are stored as map from names to values. */
struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}

type AttrMap = HashMap<String, String>;

/* Constructors for Test and Element nodes, making it easy to create
new nodes. */
fn text(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Text(data),
    }
}

fn element(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attrs,
        }),
    }
}
