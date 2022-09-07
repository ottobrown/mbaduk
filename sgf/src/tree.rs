// TODO: optimize size of SgfProp, SgfNode, and SgfTree

pub struct SgfProp {
    pub(crate) id: String,
    pub(crate) values: Vec<String>,
}

pub struct SgfNode {
    pub(crate) props: Vec<SgfProp>,
}

pub struct SgfTree {
    pub(crate) nodes: Vec<SgfNode>,
    pub(crate) children: Vec<SgfTree>,
}
