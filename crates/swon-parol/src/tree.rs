use parol_runtime::{
    parser::parse_tree_type::{
        NonTerminalEnum, TerminalEnum, TreeConstruct,
    },
    Token, ParolError,
};
use petgraph::{
    graph::{DiGraph, NodeIndex},
    visit::EdgeRef,
    Direction,
};
use std::collections::HashMap;



#[derive(Debug, Clone, Copy)]
/// A span that is only valid within the context of the input text.
pub struct InputSpan {
    /// The start of the span.
    pub start: u32,
    /// The end of the span.
    pub end: u32,
}

#[derive(Debug, Clone, Copy)]
/// A dynamic token id that provided by the user land.
pub struct DynamicTokenId(pub u32);

#[derive(Debug, Clone)]
pub enum CstNodeData<T, Nt> {
    /// A terminal node with its kind and span
    Terminal(T, InputSpan),
    /// A non-terminal node with its kind
    NonTerminal(Nt),
    /// A terminal node with its kind and dynamic token id
    DynamicToken(DynamicTokenId),
}

#[derive(Debug, Clone)]
pub struct ConcreteSyntaxTree<T, Nt> {
    pub graph: DiGraph<CstNodeData<T, Nt>, ()>,
    pub root: Option<NodeIndex>,
    node_stack: Vec<NodeIndex>,
}

impl<T, Nt> ConcreteSyntaxTree<T, Nt> {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            root: None,
            node_stack: Vec::new(),
        }
    }

    pub fn add_terminal(&mut self, kind: T, span: InputSpan) -> NodeIndex {
        let node_idx = self.graph.add_node(CstNodeData::Terminal(kind, span));
        
        if let Some(&parent) = self.node_stack.last() {
            self.graph.add_edge(parent, node_idx, ());
        } else if self.root.is_none() {
            self.root = Some(node_idx);
        }
        
        node_idx
    }

    pub fn open_non_terminal(&mut self, kind: Nt) -> NodeIndex {
        let node_idx = self.graph.add_node(CstNodeData::NonTerminal(kind));
        
        if let Some(&parent) = self.node_stack.last() {
            self.graph.add_edge(parent, node_idx, ());
        } else if self.root.is_none() {
            self.root = Some(node_idx);
        }
        
        self.node_stack.push(node_idx);
        node_idx
    }

    pub fn close_non_terminal(&mut self) -> Option<NodeIndex> {
        self.node_stack.pop()
    }

    pub fn children(&self, node: NodeIndex) -> Vec<NodeIndex> {
        self.graph
            .edges_directed(node, Direction::Outgoing)
            .map(|edge| edge.target())
            .collect()
    }

    pub fn parent(&self, node: NodeIndex) -> Option<NodeIndex> {
        self.graph
            .edges_directed(node, Direction::Incoming)
            .next()
            .map(|edge| edge.source())
    }

    pub fn node_data(&self, node: NodeIndex) -> Option<&CstNodeData<T, Nt>> {
        self.graph.node_weight(node)
    }
}

pub struct CstBuilder<T, Nt> {
    tree: ConcreteSyntaxTree<T, Nt>,
    terminal_map: HashMap<&'static str, T>,
    non_terminal_map: HashMap<&'static str, Nt>,
}

impl<T, Nt> CstBuilder<T, Nt>
where
    T: TerminalEnum + Copy,
    Nt: NonTerminalEnum + Copy,
{
    pub fn new() -> Self {
        Self {
            tree: ConcreteSyntaxTree::new(),
            terminal_map: HashMap::new(),
            non_terminal_map: HashMap::new(),
        }
    }
}

impl<'t, T, Nt> TreeConstruct<'t> for CstBuilder<T, Nt>
where
    T: TerminalEnum + Copy,
    Nt: NonTerminalEnum + Copy,
{
    type Error = ParolError;
    type Tree = ConcreteSyntaxTree<T, Nt>;

    fn open_non_terminal(
        &mut self,
        name: &'static str,
        _size_hint: Option<usize>,
    ) -> Result<(), Self::Error> {
        let kind = Nt::from_non_terminal_name(name);
        self.non_terminal_map.insert(name, kind);
        self.tree.open_non_terminal(kind);
        Ok(())
    }

    fn close_non_terminal(&mut self) -> Result<(), Self::Error> {
        self.tree.close_non_terminal();
        Ok(())
    }

    fn add_token(&mut self, token: &Token<'t>) -> Result<(), Self::Error> {
        let kind = T::from_terminal_index(token.token_type);
        let span = InputSpan {
            start: token.location.start,
            end: token.location.end,
        };
        self.tree.add_terminal(kind, span);
        Ok(())
    }

    fn build(self) -> Result<Self::Tree, Self::Error> {
        Ok(self.tree)
    }
}
