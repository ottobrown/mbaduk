use std::fmt;

// TODO: optimize size of SgfProp, SgfNode, and SgfTree

#[derive(Clone, Default, PartialEq, Debug)]
pub struct SgfProp {
    pub(crate) id: String,
    pub(crate) values: Vec<String>,
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
