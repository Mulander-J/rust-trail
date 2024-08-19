pub mod graph {
    use std::collections::HashMap;

    use graph_items::edge::Edge;
    use graph_items::macros::impl_attrs;
    use graph_items::node::Node;

    // pub mod graph_attr {
    //     use std::collections::HashMap;
    //     pub trait WithAttrs {
    //         fn attrs_mut(&mut self) -> &mut HashMap<String, String>;
    //         fn with_attrs(mut self, attrs: &[(&'static str, &'static str)]) -> Self
    //         where
    //             Self: Sized,
    //         {
    //             attrs.iter().for_each(|&(name, value)| {
    //                 self.attrs_mut().insert(name.to_string(), value.to_string());
    //             });
    //             self
    //         }
    //         fn attr(&self, name: &str) -> Option<&str> {
    //             self.attrs().get(name).map(|v| v.as_ref())
    //         }
    //         fn attrs(&self) -> &HashMap<String, String>;
    //     }
    // }

    pub mod graph_items {
        pub mod macros {
            macro_rules! impl_attrs {
                () => {
                    pub fn attr(&self, key: &str) -> Option<&str> {
                        self.attrs.get(key).map(|s| s.as_ref())
                    }
                    pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
                        Self {
                            attrs: attrs
                                .iter()
                                .map(|(k, v)| (k.to_string(), v.to_string()))
                                .collect(),
                            ..self
                        }
                    }
                };
            }
            pub(crate) use impl_attrs;
        }
        pub mod node {
            use std::collections::HashMap;

            use super::macros::impl_attrs;

            #[derive(Clone, PartialEq, Debug)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }
            impl Node {
                pub fn new(name: &str) -> Self {
                    Self {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    }
                }
                impl_attrs!();
            }
        }
        pub mod edge {
            use std::collections::HashMap;

            use super::macros::impl_attrs;

            #[derive(Clone, PartialEq, Debug)]
            pub struct Edge {
                pub src: String,
                pub dst: String,
                pub attrs: HashMap<String, String>,
            }
            impl Edge {
                pub fn new(src: &str, dst: &str) -> Self {
                    Edge {
                        src: src.to_string(),
                        dst: dst.to_string(),
                        attrs: HashMap::new(),
                    }
                }
                impl_attrs!();
            }
        }
    }
    #[derive(Clone, PartialEq, Debug)]
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
        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes = nodes.to_vec();
            self
        }
        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges = edges.to_vec();
            self
        }
        pub fn node(&self, name: &str) -> Option<&Node> {
            self.get_node(name)
        }
        pub fn get_node(&self, name: &str) -> Option<&Node> {
            // self.nodes.iter().filter(|n| n.name == name).nth(0)
            self.nodes.iter().find(|n| n.name == name)
        }
        impl_attrs!();
    }
}
