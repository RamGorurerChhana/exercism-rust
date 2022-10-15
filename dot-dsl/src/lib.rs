pub mod graph {
    pub mod graph_items {
        pub mod edge {
            #[derive(Debug, Clone, PartialEq)]
            pub struct Edge {
                to: String,
                from: String,
                attrs: Vec<(String, String)>,
            }

            impl Edge {
                pub fn new(to: &str, from: &str) -> Self {
                    Self {
                        to: to.into(),
                        from: from.into(),
                        attrs: vec![],
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    let attrs = attrs
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect::<Vec<_>>();
                    self.attrs = attrs;
                    self
                    // Self { attrs, ..self }
                }

                pub fn attr(&self, key: &str) -> Option<&str> {
                    for (k, v) in &self.attrs {
                        if *k == key {
                            return Some(v);
                        }
                    }

                    None
                }
            }
        }

        pub mod node {
            #[derive(Debug, Clone, PartialEq)]
            pub struct Node {
                pub label: String,
                attrs: Vec<(String, String)>,
            }

            impl Node {
                pub fn new(label: &str) -> Self {
                    Self {
                        label: label.to_string(),
                        attrs: vec![],
                    }
                }

                pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
                    let attrs = attrs
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect::<Vec<_>>();
                    Self {
                        label: self.label,
                        attrs,
                    }
                }

                pub fn attr(&self, key: &str) -> Option<&str> {
                    for (k, v) in &self.attrs {
                        if *k == key {
                            return Some(v);
                        }
                    }

                    None
                }
            }
        }
    }

    use crate::graph::graph_items::edge::Edge;
    use crate::graph::graph_items::node::Node;
    use std::collections::HashMap;

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                nodes: vec![],
                edges: vec![],
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(self, nodes: &Vec<Node>) -> Self {
            Self {
                nodes: nodes.to_vec(),
                ..self
            }
        }

        pub fn with_edges(self, edges: &Vec<Edge>) -> Self {
            Self {
                edges: edges.to_vec(),
                ..self
            }
        }

        pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
            let mut map = HashMap::new();
            for (k, v) in attrs.iter() {
                map.insert(k.to_string(), v.to_string());
            }
            Self { attrs: map, ..self }
        }
        pub fn node(&self, label: &str) -> Option<&Node> {
            for node in &self.nodes {
                if node.label == label {
                    return Some(node);
                }
            }

            None
        }
    }
}
