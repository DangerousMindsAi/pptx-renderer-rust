use std::collections::HashMap;
use quick_xml::events::Event;
use quick_xml::Reader;

#[derive(Debug, Clone)]
pub struct XmlNode {
    pub tag: String,
    pub attributes: HashMap<String, String>,
    pub children: Vec<XmlNode>,
    pub text: Option<String>,
}

impl XmlNode {
    pub fn parse(xml: &str) -> Result<Self, String> {
        let mut reader = Reader::from_str(xml);
        // reader.config_mut().trim_text(true);
        
        let mut stack = Vec::new();
        let mut root = None;

        loop {
            match reader.read_event() {
                Ok(Event::Start(ref e)) => {
                    let mut attributes = HashMap::new();
                    for attr in e.attributes() {
                        if let Ok(a) = attr {
                            let key = String::from_utf8_lossy(a.key.as_ref()).to_string();
                            let value = String::from_utf8_lossy(&a.value).to_string();
                            attributes.insert(key, value);
                        }
                    }
                    let node = XmlNode {
                        tag: String::from_utf8_lossy(e.name().as_ref()).to_string(),
                        attributes,
                        children: Vec::new(),
                        text: None,
                    };
                    stack.push(node);
                }
                Ok(Event::Empty(ref e)) => {
                    let mut attributes = HashMap::new();
                    for attr in e.attributes() {
                        if let Ok(a) = attr {
                            let key = String::from_utf8_lossy(a.key.as_ref()).to_string();
                            let value = String::from_utf8_lossy(&a.value).to_string();
                            attributes.insert(key, value);
                        }
                    }
                    let node = XmlNode {
                        tag: String::from_utf8_lossy(e.name().as_ref()).to_string(),
                        attributes,
                        children: Vec::new(),
                        text: None,
                    };
                    if let Some(parent) = stack.last_mut() {
                        parent.children.push(node);
                    } else if root.is_none() {
                        root = Some(node);
                    }
                }
                Ok(Event::End(_)) => {
                    if let Some(node) = stack.pop() {
                        if let Some(parent) = stack.last_mut() {
                            parent.children.push(node);
                        } else {
                            root = Some(node);
                        }
                    }
                }
                Ok(Event::Text(e)) => {
                    if let Some(node) = stack.last_mut() {
                        let mut txt = e.unescape().unwrap_or_default().to_string();
                        if node.tag != "a:t" && node.tag != "p:t" {
                            txt = txt.trim().to_string();
                        }
                        if !txt.is_empty() {
                            if let Some(ref mut existing) = node.text {
                                existing.push_str(&txt);
                            } else {
                                node.text = Some(txt);
                            }
                        }
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(e.to_string()),
                _ => (),
            }
        }
        
        root.ok_or_else(|| "Empty XML".to_string())
    }

    pub fn child(&self, tag: &str) -> Option<&XmlNode> {
        self.children.iter().find(|c| {
            c.tag == tag || c.tag.ends_with(&format!(":{}", tag))
        })
    }

    pub fn children(&self, tag: &str) -> impl Iterator<Item = &XmlNode> {
        let tag = tag.to_string();
        self.children.iter().filter(move |c| {
            c.tag == tag || c.tag.ends_with(&format!(":{}", tag))
        })
    }

    pub fn attr(&self, key: &str) -> Option<&String> {
        self.attributes.get(key)
    }

    pub fn num_attr(&self, key: &str) -> Option<f64> {
        self.attr(key).and_then(|v| v.parse().ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xmlnode_parse_and_attributes() {
        let xml = r#"<root id="123" num="45.6" empty=""></root>"#;
        let node = XmlNode::parse(xml).unwrap();
        
        assert_eq!(node.tag, "root");
        assert_eq!(node.attr("id").map(|s| s.as_str()), Some("123"));
        assert_eq!(node.num_attr("num"), Some(45.6));
        assert_eq!(node.attr("empty").map(|s| s.as_str()), Some(""));
        assert_eq!(node.attr("missing"), None);
    }

    #[test]
    fn test_xmlnode_children_iterators() {
        let xml = r#"
            <root>
                <child1 id="1"/>
                <child2>text</child2>
                <child1 id="2"/>
            </root>
        "#;
        let node = XmlNode::parse(xml).unwrap();
        
        // Test child
        let c1 = node.child("child1").unwrap();
        assert_eq!(c1.attr("id").map(|s| s.as_str()), Some("1"));
        
        let c2 = node.child("child2").unwrap();
        assert_eq!(c2.text.as_deref(), Some("text"));
        
        // Test children iterator
        let c1_children: Vec<&XmlNode> = node.children("child1").collect();
        assert_eq!(c1_children.len(), 2);
        assert_eq!(c1_children[0].attr("id").map(|s| s.as_str()), Some("1"));
        assert_eq!(c1_children[1].attr("id").map(|s| s.as_str()), Some("2"));
        
        assert!(node.child("missing").is_none());
    }
}
