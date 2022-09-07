// TODO: optimize size of SgfProp, SgfNode, and SgfTree

pub struct SgfProp {
    id: String,
    values: Vec<String>,
}

pub struct SgfNode {
    props: Vec<SgfProp>,
}

pub struct SgfTree {
    nodes: Vec<SgfNode>,
    children: Vec<SgfTree>,
}
