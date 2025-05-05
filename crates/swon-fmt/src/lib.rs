use swon_parol::{
    tree::{CstNodeData, CstNodeId},
    Cst, CstNode,
};

pub fn fmt(input: &str, cst: &mut Cst) {
    let mut formatter = Formatter::new();
    formatter.walk_node(cst, cst.root());
}

#[derive(Debug, Clone, Default)]
pub struct Formatter {
    indent_queue: Vec<usize>,
}

impl Formatter {
    pub fn new() -> Self {
        Self {
            indent_queue: vec![],
        }
    }

    pub fn walk_node(&mut self, cst: &mut Cst, cst_id: CstNodeId) {
        let node = cst.node_data(cst_id).expect("node is not found");
        match node {
            CstNodeData::Terminal(_, input_span) => todo!(),
            CstNodeData::NonTerminal(_) => todo!(),
            CstNodeData::DynamicToken(_, dynamic_token_id) => todo!(),
        }
    }
}
