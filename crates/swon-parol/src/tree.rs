use parol_runtime::{
    parser::parse_tree_type::{
        AstNode, ExpectedChildren, ExpectedChildrenKinds, NodeKind, NonTerminalEnum, TerminalEnum, TreeConstruct,
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
/// A parse tree that is associated with a grammar.
pub enum SynTree2<T, Nt> {
    /// A terminal node.
    Terminal(SynTreeTerminal<T>),
    /// A non-terminal node.
    NonTerminal(SynTreeNonTerminal<Nt>),
}

impl<T: Copy, Nt: Copy> SynTree2<T, Nt> {
    /// The kind of the node.
    pub fn kind(&self) -> NodeKind<T, Nt> {
        match self {
            SynTree2::Terminal(data) => NodeKind::Terminal(data.kind),
            SynTree2::NonTerminal(data) => NodeKind::NonTerminal(data.kind),
        }
    }
}

impl<T, Nt> std::fmt::Display for SynTree2<T, Nt>
where
    T: std::fmt::Display,
    Nt: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SynTree2::Terminal(t) => write!(f, "{}", t),
            SynTree2::NonTerminal(n) => write!(f, "{}", n),
        }
    }
}

#[derive(Debug, Clone, Copy)]
/// A terminal node.
pub struct SynTreeTerminal<T> {
    /// The kind of the terminal.
    pub kind: T,
    /// The data of the terminal.
    pub data: TerminalData,
}

impl<T> std::fmt::Display for SynTreeTerminal<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

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

#[derive(Debug, Clone, Copy)]
/// The data of the terminal.
pub enum TerminalData {
    /// A terminal that is associated with an input span.
    Input(InputSpan),
    /// A terminal that is associated with a dynamic token id.
    Dynamic(DynamicTokenId),
}

#[derive(Debug, Clone, Copy)]
/// A non-terminal node.
pub struct SynTreeNonTerminal<Nt> {
    /// The kind of the non-terminal.
    pub kind: Nt,
}

impl<Nt> std::fmt::Display for SynTreeNonTerminal<Nt>
where
    Nt: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl<'t, T, Nt> AstNode<'t> for SynTree2<T, Nt>
where
    T: TerminalEnum,
    Nt: NonTerminalEnum,
{
    fn from_token(token: &Token<'t>) -> Self {
        SynTree2::Terminal(SynTreeTerminal {
            kind: T::from_terminal_index(token.token_type),
            data: TerminalData::Input(InputSpan {
                start: token.location.start,
                end: token.location.end,
            }),
        })
    }
    fn from_non_terminal(name: &'static str) -> Self {
        SynTree2::NonTerminal(SynTreeNonTerminal {
            kind: Nt::from_non_terminal_name(name),
        })
    }
}

impl<T, Nt> ExpectedChildren<T, Nt> for SynTree2<T, Nt>
where
    Nt: ExpectedChildren<T, Nt>,
{
    fn expected_children(&self) -> ExpectedChildrenKinds<T, Nt> {
        match self {
            SynTree2::Terminal(_) => ExpectedChildrenKinds::Sequence(&[]),
            SynTree2::NonTerminal(nt) => nt.kind.expected_children(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum CstNodeData<T, Nt> {
    /// A terminal node with its kind and span
    Terminal(T, InputSpan),
    /// A non-terminal node with its kind
    NonTerminal(Nt),
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
