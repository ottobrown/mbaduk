use std::fmt;

// TODO: optimize size of SgfProp, SgfNode, and SgfTree

#[derive(Clone, Default, PartialEq, Debug)]
pub struct SgfProp {
    pub(crate) id: String,
    pub(crate) values: Vec<String>,
}
impl SgfProp {
    pub fn new<S: Into<String>>(id: impl Into<String>, values: impl Iterator<Item = S>) -> Self {
        Self {
            id: id.into(),
            values: values.map(|s| s.into()).collect(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn values(&self) -> &Vec<String> {
        &self.values
    }
}

impl fmt::Display for SgfProp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)?;

        for v in &self.values {
            write!(f, "[{}]", v)?;
        }

        Ok(())
    }
}

#[derive(Clone, Default, PartialEq, Debug)]
pub struct SgfNode {
    pub(crate) props: Vec<SgfProp>,
}
impl SgfNode {
    pub fn new(props: impl Iterator<Item = SgfProp>) -> Self {
        Self {
            props: props.collect(),
        }
    }

    pub fn props(&self) -> &Vec<SgfProp> {
        &self.props
    }
}

impl fmt::Display for SgfNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, ";")?;

        for p in &self.props {
            write!(f, "{p}")?;
        }

        Ok(())
    }
}

#[derive(Clone, Default, PartialEq, Debug)]
pub struct SgfTree {
    pub(crate) nodes: Vec<SgfNode>,
    pub(crate) children: Vec<SgfTree>,
}
impl SgfTree {
    pub fn new(nodes: impl Iterator<Item = SgfNode>, children: impl Iterator<Item = SgfTree>) -> Self {
        Self {
            nodes: nodes.collect(),
            children: children.collect(),
        }
    }

    pub fn nodes(&self) -> &Vec<SgfNode> {
        &self.nodes
    }

    pub fn children(&self) -> &Vec<SgfTree> {
        &self.children
    }
}

impl fmt::Display for SgfTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;

        for n in &self.nodes {
            write!(f, "{n}")?;
        }

        for c in &self.children {
            write!(f, "{c}")?;
        }

        write!(f, ")")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_prop() {
        let prop = SgfProp {
            id: String::from("AB"),
            values: vec![String::from("cd"), String::from("ef")],
        };

        assert_eq!(format!("{}", prop), String::from("AB[cd][ef]"));
    }

    #[test]
    fn format_node() {
        let node = SgfNode {
            props: vec![
                SgfProp {
                    id: String::from("AB"),
                    values: vec![String::from("cd"), String::from("ef")],
                },
                SgfProp {
                    id: String::from("AW"),
                    values: vec![String::from("aa"), String::from("bb")],
                },
            ],
        };

        assert_eq!(format!("{}", node), String::from(";AB[cd][ef]AW[aa][bb]"));
    }

    #[test]
    fn format_tree() {
        let tree = SgfTree {
            nodes: vec![
                SgfNode {
                    props: vec![
                        SgfProp {
                            id: String::from("AB"),
                            values: vec![String::from("cd"), String::from("ef")],
                        },
                        SgfProp {
                            id: String::from("AW"),
                            values: vec![String::from("aa"), String::from("bb")],
                        },
                    ],
                },

                SgfNode {
                    props: vec![SgfProp { id: String::from("B"), values: vec![String::from("qq")] }],
                }
            ],

            children: vec![
                SgfTree {
                    nodes: vec![
                        SgfNode {
                            props: vec![SgfProp { id: String::from("W"), values: vec![String::from("aq")] }],
                        }
                    ],
                    children: Vec::new(),
                },

                SgfTree {
                    nodes: vec![
                        SgfNode {
                            props: vec![SgfProp { id: String::from("W"), values: vec![String::from("bq")] }],
                        }
                    ],
                    children: Vec::new(),
                }
            ],
        };

        assert_eq!(format!("{tree}"), String::from("(;AB[cd][ef]AW[aa][bb];B[qq](;W[aq])(;W[bq]))"))
    }
}
