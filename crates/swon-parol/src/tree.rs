mod span;

use parol_runtime::{
    parser::parse_tree_type::{NonTerminalEnum, TerminalEnum, TreeConstruct},
    ParolError, Token,
};
use petgraph::{
    graph::{DiGraph, NodeIndex},
    visit::EdgeRef,
    Direction,
};
use std::collections::BTreeMap;

pub use span::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
/// A dynamic token id that provided by the user land.
pub struct DynamicTokenId(pub u32);

#[derive(Debug, Clone)]
pub enum CstNodeData<T, Nt> {
    /// A terminal node with its kind and span
    Terminal(T, InputSpan),
    /// A non-terminal node with its kind
    NonTerminal(Nt),
    /// A terminal node with its kind and dynamic token id
    DynamicToken(T, DynamicTokenId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct CstNodeId(pub NodeIndex);

/// A generic concrete syntax tree that doesn't know about SWON-specific ordering issues
#[derive(Debug, Clone)]
pub struct ConcreteSyntaxTree<T, Nt> {
    graph: DiGraph<CstNodeData<T, Nt>, ()>,
    dynamic_tokens: BTreeMap<DynamicTokenId, String>,
    root: CstNodeId,
}

impl<T, Nt> ConcreteSyntaxTree<T, Nt> {
    pub fn root(&self) -> CstNodeId {
        self.root
    }

    pub fn add_node(&mut self, data: CstNodeData<T, Nt>) -> CstNodeId {
        let node = self.graph.add_node(data);
        CstNodeId(node)
    }

    pub fn add_edge(&mut self, from: CstNodeId, to: CstNodeId) {
        self.graph.add_edge(from.0, to.0, ());
    }

    pub fn get_children(&self, node: CstNodeId) -> Vec<CstNodeId> {
        self.graph
            .edges_directed(node.0, Direction::Outgoing)
            .map(|edge| CstNodeId(edge.target()))
            .collect()
    }

    pub fn parent(&self, node: CstNodeId) -> Option<CstNodeId> {
        self.graph
            .edges_directed(node.0, Direction::Incoming)
            .next()
            .map(|edge| CstNodeId(edge.source()))
    }

    pub fn node_data(&self, node: CstNodeId) -> Option<&CstNodeData<T, Nt>> {
        self.graph.node_weight(node.0)
    }

    /// Get all children of a node in source order - this handles the reversed nature of lists
    /// by returning them in reverse order of insertion
    pub fn children(&self, node: CstNodeId) -> Vec<CstNodeId> {
        let mut children = self.get_children(node);
        // Reverse to get source order for lists
        children.reverse();
        children
    }

    pub fn write(&self, input: &str, w: &mut impl std::fmt::Write) -> Result<(), std::fmt::Error> {
        let mut visitor = FormatVisitor::new(input, w);
        self.visit_tree(&mut visitor, self.root())?;
        Ok(())
    }

    pub fn inspect(&self, input: &str, w: &mut impl std::fmt::Write) -> Result<(), std::fmt::Error>
    where
        T: std::fmt::Debug,
        Nt: std::fmt::Debug,
    {
        let mut visitor = InspectVisitor::new(input, w);
        self.visit_tree(&mut visitor, self.root())?;
        Ok(())
    }

    fn visit_tree<V: TreeVisitor<T, Nt>>(
        &self,
        visitor: &mut V,
        node_id: CstNodeId,
    ) -> Result<(), V::Error> {
        visitor.visit_node(
            self,
            node_id,
            self.node_data(node_id).expect(
                "This is private function and should always be called with a valid node id",
            ),
        )?;
        Ok(())
    }
}

/// SWON-specific tree builder that handles the peculiarities of the SWON grammar,
/// particularly the reversed ordering of list elements
#[derive(Debug, Clone)]
pub struct CstBuilder<T, Nt> {
    tree: DiGraph<CstNodeData<T, Nt>, ()>,
    node_stack: Vec<NodeIndex>,
    root_node_candidate: Option<NodeIndex>,
}

impl<T, Nt> Default for CstBuilder<T, Nt>
where
    T: TerminalEnum + Copy,
    Nt: NonTerminalEnum + Copy,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, Nt> CstBuilder<T, Nt>
where
    T: TerminalEnum + Copy,
    Nt: NonTerminalEnum + Copy,
{
    pub fn new() -> Self {
        Self {
            tree: DiGraph::new(),
            node_stack: Vec::new(),
            root_node_candidate: None,
        }
    }

    // Adds a terminal node to the current non-terminal in the stack
    fn add_terminal_node(&mut self, kind: T, span: InputSpan) -> NodeIndex {
        let node = self.tree.add_node(CstNodeData::Terminal(kind, span));

        if let Some(&parent) = self.node_stack.last() {
            self.tree.add_edge(parent, node, ());
        } else {
            // This is a root level node
            self.root_node_candidate = Some(node);
        }

        node
    }

    // Opens a new non-terminal and adds it to the stack
    fn open_non_terminal_node(&mut self, kind: Nt) -> NodeIndex {
        let node = self.tree.add_node(CstNodeData::NonTerminal(kind));

        if let Some(&parent) = self.node_stack.last() {
            self.tree.add_edge(parent, node, ());
        } else {
            // This is a root level node
            self.root_node_candidate = Some(node);
        }

        self.node_stack.push(node);
        node
    }

    // Closes the current non-terminal
    fn close_non_terminal_node(&mut self) -> Option<NodeIndex> {
        self.node_stack.pop()
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
        self.open_non_terminal_node(kind);
        Ok(())
    }

    fn close_non_terminal(&mut self) -> Result<(), Self::Error> {
        self.close_non_terminal_node();
        Ok(())
    }

    fn add_token(&mut self, token: &Token<'t>) -> Result<(), Self::Error> {
        let kind = T::from_terminal_index(token.token_type);
        let span = InputSpan {
            start: token.location.start,
            end: token.location.end,
        };
        self.add_terminal_node(kind, span);
        Ok(())
    }

    fn build(self) -> Result<Self::Tree, Self::Error> {
        Ok(ConcreteSyntaxTree {
            graph: self.tree,
            dynamic_tokens: BTreeMap::new(),
            root: CstNodeId(self.root_node_candidate.expect("Root node always provided")),
        })
    }
}

pub trait TreeVisitor<T, Nt> {
    type Error;
    fn visit_node(
        &mut self,
        tree: &ConcreteSyntaxTree<T, Nt>,
        node_id: CstNodeId,
        data: &CstNodeData<T, Nt>,
    ) -> Result<(), Self::Error>;
}

struct FormatVisitor<'f, 't> {
    input: &'t str,
    f: &'f mut dyn std::fmt::Write,
}

impl<'f, 't> FormatVisitor<'f, 't> {
    fn new(input: &'t str, f: &'f mut dyn std::fmt::Write) -> Self {
        Self { input, f }
    }
}

impl<T, Nt> TreeVisitor<T, Nt> for FormatVisitor<'_, '_> {
    type Error = std::fmt::Error;
    fn visit_node(
        &mut self,
        tree: &ConcreteSyntaxTree<T, Nt>,
        node_id: CstNodeId,
        data: &CstNodeData<T, Nt>,
    ) -> Result<(), Self::Error> {
        match data {
            CstNodeData::Terminal(_kind, span) => {
                write!(
                    self.f,
                    "{}",
                    &self.input[span.start as usize..span.end as usize]
                )?;
            }
            CstNodeData::NonTerminal(_) => {}
            CstNodeData::DynamicToken(_kind, dynamic_token_id) => {
                write!(self.f, "{}", &tree.dynamic_tokens[dynamic_token_id])?;
            }
        }
        for child in tree.children(node_id) {
            if let Some(child_data) = tree.node_data(child) {
                self.visit_node(tree, child, child_data)?;
            }
        }
        Ok(())
    }
}

struct InspectVisitor<'f, 't> {
    input: &'t str,
    indent: usize,
    f: &'f mut dyn std::fmt::Write,
}

impl<'f, 't> InspectVisitor<'f, 't> {
    fn new(input: &'t str, f: &'f mut dyn std::fmt::Write) -> Self {
        Self {
            input,
            f,
            indent: 0,
        }
    }
}

impl<T, Nt> TreeVisitor<T, Nt> for InspectVisitor<'_, '_>
where
    T: std::fmt::Debug,
    Nt: std::fmt::Debug,
{
    type Error = std::fmt::Error;
    fn visit_node(
        &mut self,
        tree: &ConcreteSyntaxTree<T, Nt>,
        node_id: CstNodeId,
        data: &CstNodeData<T, Nt>,
    ) -> Result<(), Self::Error> {
        match data {
            CstNodeData::Terminal(kind, input_span) => writeln!(
                self.f,
                "{}{} ({:?})",
                " ".repeat(self.indent),
                &self.input[input_span.start as usize..input_span.end as usize]
                    .replace("\n", "\\n")
                    .replace("\t", "\\t")
                    .replace(" ", "_"),
                kind,
            )?,
            CstNodeData::NonTerminal(kind) => {
                writeln!(self.f, "{}{:?}", " ".repeat(self.indent), kind)?
            }
            CstNodeData::DynamicToken(_kind, token_id) => writeln!(
                self.f,
                "{}{:?} (dynamic)",
                " ".repeat(self.indent),
                token_id
            )?,
        }

        // Increase indent for children
        self.indent += 2;
        for child in tree.children(node_id) {
            if let Some(child_data) = tree.node_data(child) {
                self.visit_node(tree, child, child_data)?;
            }
        }
        // Restore indent after processing children
        self.indent -= 2;

        Ok(())
    }
}
