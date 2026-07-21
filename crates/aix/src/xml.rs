use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::collections::HashMap;

/// A lightweight XML/SFC AST node used internally by `aix`.
///
/// This type represents the parsed output of `parse_xml()` and `parse_sfc()`,
/// and is reused by AIX for page config extraction, template inspection, and
/// page size analysis.
#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    /// A regular element node.
    Element {
        /// The tag name, such as `view`, `script`, or `page`.
        name: String,
        /// The attribute map for this element.
        ///
        /// Keys are attribute names and values are attribute values. Valueless
        /// attributes are stored as empty strings.
        attributes: HashMap<String, String>,
        /// The direct child nodes of this element.
        children: Vec<Node>,
    },
    /// A text node containing raw text content.
    Text(String),
}

/// Parses XML/WXML-like text into a list of top-level nodes.
///
/// This function uses `quick-xml` as a streaming parser and builds a simplified
/// tree structure:
///
/// - element nodes become `Node::Element`
/// - non-empty text becomes `Node::Text`
/// - loosely matched end tags are tolerated to support less strict template input
///
/// # Parameters
///
/// - `content`: The XML text to parse, typically a `.wxml` file or a template fragment.
///
/// # Returns
///
/// - `Ok(Vec<Node>)`: The parsed top-level node list.
/// - `Err(String)`: A parse error message containing the approximate position and
///   the underlying parser error.
pub fn parse_xml(content: &str) -> Result<Vec<Node>, String> {
    let mut reader = Reader::from_str(content);
    reader.config_mut().trim_text(true);
    // Allow unmatched ends in case HTML is slightly malformed or valueless attributes.
    reader.config_mut().allow_unmatched_ends = true;

    let mut stack: Vec<Node> = vec![Node::Element {
        name: "root".to_string(),
        attributes: HashMap::new(),
        children: Vec::new(),
    }];

    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                let mut attributes = HashMap::new();

                for a in e.html_attributes().flatten() {
                    let key = String::from_utf8_lossy(a.key.as_ref()).to_string();
                    let value = String::from_utf8_lossy(&a.value).to_string();
                    attributes.insert(key, value);
                }

                stack.push(Node::Element {
                    name,
                    attributes,
                    children: Vec::new(),
                });
            }
            Ok(Event::Empty(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                let mut attributes = HashMap::new();

                for a in e.html_attributes().flatten() {
                    let key = String::from_utf8_lossy(a.key.as_ref()).to_string();
                    let value = String::from_utf8_lossy(&a.value).to_string();
                    attributes.insert(key, value);
                }

                let node = Node::Element {
                    name,
                    attributes,
                    children: Vec::new(),
                };

                if let Some(Node::Element { children, .. }) = stack.last_mut() {
                    children.push(node);
                }
            }
            Ok(Event::End(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                if stack.len() > 1 {
                    let mut found = false;
                    if let Some(Node::Element {
                        name: node_name, ..
                    }) = stack.last()
                    {
                        if node_name == &name {
                            found = true;
                        }
                    }

                    if found {
                        let node = stack.pop().unwrap();
                        if let Some(Node::Element { children, .. }) = stack.last_mut() {
                            children.push(node);
                        }
                    } else {
                        let mut pop_count = 0;
                        for i in (1..stack.len()).rev() {
                            if let Node::Element { name: n, .. } = &stack[i] {
                                if n == &name {
                                    pop_count = stack.len() - i;
                                    break;
                                }
                            }
                        }
                        if pop_count > 0 {
                            let mut popped = Vec::new();
                            for _ in 0..pop_count {
                                popped.push(stack.pop().unwrap());
                            }
                            let node = popped.pop().unwrap();
                            if let Node::Element { children, .. } = stack.last_mut().unwrap() {
                                children.push(node);
                            }
                        }
                    }
                }
            }
            Ok(Event::Text(e)) => {
                let text = String::from_utf8_lossy(e.as_ref()).to_string();
                if !text.trim().is_empty() {
                    if let Some(Node::Element { children, .. }) = stack.last_mut() {
                        children.push(Node::Text(text));
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                return Err(format!(
                    "Error at position {}: {:?}",
                    reader.buffer_position(),
                    e
                ));
            }
            _ => (),
        }
        buf.clear();
    }

    while stack.len() > 1 {
        let node = stack.pop().unwrap();
        if let Some(Node::Element { children, .. }) = stack.last_mut() {
            children.push(node);
        }
    }

    if let Node::Element { children, .. } = stack.pop().unwrap() {
        Ok(children)
    } else {
        Ok(Vec::new())
    }
}

/// Parses `.ink` single-file component text and extracts top-level structure blocks.
///
/// This is a simplified parser tailored for AIX analysis. It does not attempt to
/// implement full XML semantics. Instead, it focuses on the following raw-text
/// blocks:
///
/// - `script`
/// - `style`
/// - `page`
/// - `template`
///
/// For these tags, the parser preserves their attributes and stores the raw inner
/// content as a single `Node::Text` child so later stages can directly extract
/// config, template, and style text.
///
/// # Parameters
///
/// - `content`: The full contents of a `.ink` file.
///
/// # Returns
///
/// - `Ok(Vec<Node>)`: The extracted top-level SFC blocks.
/// - `Err(String)`: This return type is kept for API consistency. In normal usage,
///   this simplified parser primarily returns results through `Ok(...)`.
pub fn parse_sfc(content: &str) -> Result<Vec<Node>, String> {
    // Treat script/style/page/template as raw text blocks.
    let mut nodes = Vec::new();
    let mut current_pos = 0;

    while current_pos < content.len() {
        if let Some(start_idx) = content[current_pos..].find('<') {
            let start = current_pos + start_idx;
            if let Some(end_idx) = content[start..].find('>') {
                let end = start + end_idx;
                let tag_content = &content[start + 1..end];

                let mut parts = tag_content.split_whitespace();
                if let Some(name) = parts.next() {
                    if name.starts_with('/') || name == "!" {
                        current_pos = end + 1;
                        continue;
                    }

                    let mut attributes = HashMap::new();
                    for part in parts {
                        if let Some((k, v)) = part.split_once('=') {
                            let val = v.trim_matches('"').trim_matches('\'').to_string();
                            attributes.insert(k.to_string(), val);
                        } else {
                            attributes.insert(part.to_string(), String::new());
                        }
                    }

                    if name == "script" || name == "style" || name == "page" || name == "template" {
                        let end_tag = format!("</{}>", name);
                        if let Some(close_idx) = content[end + 1..].find(&end_tag) {
                            let inner_content = &content[end + 1..end + 1 + close_idx];
                            nodes.push(Node::Element {
                                name: name.to_string(),
                                attributes,
                                children: vec![Node::Text(inner_content.to_string())],
                            });
                            current_pos = end + 1 + close_idx + end_tag.len();
                            continue;
                        }
                    }
                }
                current_pos = end + 1;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    Ok(nodes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valueless_attributes() {
        let xml = "<script setup><script def></script></script>";
        let nodes = parse_xml(xml).unwrap();
        assert_eq!(nodes.len(), 1);
        if let Node::Element {
            name,
            attributes,
            children,
        } = &nodes[0]
        {
            assert_eq!(name, "script");
            assert_eq!(attributes.get("setup").unwrap(), "");
            assert_eq!(children.len(), 1);
        } else {
            panic!("Expected element");
        }
    }
}
