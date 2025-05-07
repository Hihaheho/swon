mod span;

use parol_runtime::{
    ParolError, Token,
    parser::parse_tree_type::{NodeKind, NonTerminalEnum, TerminalEnum, TreeConstruct},
};
use petgraph::{
    Direction,
    graph::{DiGraph, NodeIndex},
    visit::EdgeRef,
};
use std::collections::BTreeMap;
use thiserror::Error;

pub use span::*;

use crate::nodes::{NonTerminalKind, TerminalKind};

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

impl<T, Nt> CstNodeData<T, Nt>
where
    T: Copy,
    Nt: Copy,
{
    pub fn node_kind(&self) -> NodeKind<T, Nt> {
        match self {
            CstNodeData::Terminal(kind, _) => NodeKind::Terminal(*kind),
            CstNodeData::NonTerminal(kind) => NodeKind::NonTerminal(*kind),
            CstNodeData::DynamicToken(kind, _) => NodeKind::Terminal(*kind),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct CstNodeId(pub NodeIndex);

impl std::fmt::Display for CstNodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.index())
    }
}

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

    pub fn has_no_children(&self, node: CstNodeId) -> bool {
        self.graph
            .edges_directed(node.0, Direction::Outgoing)
            .next()
            .is_none()
    }

    pub fn children(&self, node: CstNodeId) -> impl Iterator<Item = CstNodeId> {
        self.graph
            .edges_directed(node.0, Direction::Outgoing)
            .map(|edge| CstNodeId(edge.target()))
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

impl TerminalKind {
    fn auto_ws_is_off(&self, index: usize) -> bool {
        // TODO: support CodeBlockDelimitor it's also used for beginning of CodeBlock and it's non-terminal so need more analysis to handle this case
        matches!(
            (self, index),
            (
                TerminalKind::Whitespace
                    | TerminalKind::Newline
                    | TerminalKind::InStr
                    | TerminalKind::Text
                    | TerminalKind::CodeBlockLine
                    | TerminalKind::Code,
                _
            ) | (TerminalKind::Quote, 2)
        )
    }
}

impl ConcreteSyntaxTree<TerminalKind, NonTerminalKind> {
    pub fn collect_nodes<const N: usize>(
        &self,
        parent: CstNodeId,
        nodes: [NodeKind<TerminalKind, NonTerminalKind>; N],
    ) -> Result<[CstNodeId; N], ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let mut children = self.children(parent);
        let mut result = Vec::with_capacity(N);
        'outer: for expected_kind in nodes {
            'inner: for (idx, child) in children.by_ref().enumerate() {
                let child_data = self
                    .node_data(child)
                    .ok_or(ViewConstructionError::NodeIdNotFound { node: child })?;
                match child_data {
                    CstNodeData::Terminal(kind, _) | CstNodeData::DynamicToken(kind, _) => {
                        if NodeKind::Terminal(*kind) == expected_kind {
                            result.push(child);
                            continue 'outer;
                        } else if kind.is_builtin_whitespace() || kind.is_builtin_new_line() {
                            if kind.auto_ws_is_off(idx) {
                                return Err(ViewConstructionError::UnexpectedTerminal {
                                    node: child,
                                    terminal: *kind,
                                });
                            }
                            continue 'inner;
                        } else {
                            return Err(ViewConstructionError::UnexpectedTerminal {
                                node: child,
                                terminal: *kind,
                            });
                        }
                    }
                    CstNodeData::NonTerminal(kind) => {
                        if NodeKind::NonTerminal(*kind) == expected_kind {
                            result.push(child);
                            continue 'outer;
                        } else {
                            return Err(ViewConstructionError::UnexpectedNonTerminal {
                                node: child,
                                non_terminal: *kind,
                            });
                        }
                    }
                }
            }
            return Err(ViewConstructionError::UnexpectedEndOfChildren { parent });
        }
        Ok(result
            .try_into()
            .expect("Result should have the same length as nodes"))
    }
}

/// SWON-specific tree builder that handles the peculiarities of the SWON grammar,
/// particularly the reversed ordering of list elements
#[derive(Debug, Clone)]
pub struct CstBuilder<T, Nt> {
    tree: DiGraph<CstNodeData<T, Nt>, ()>,
    node_stack: Vec<NodeStackItem>,
    root_node_candidate: Option<NodeIndex>,
}

#[derive(Debug, Clone)]
struct NodeStackItem {
    node: CstNodeId,
    children: Vec<CstNodeId>,
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

        if let Some(parent) = self.node_stack.last_mut() {
            parent.children.push(CstNodeId(node));
        } else {
            // This is a root level node
            self.root_node_candidate = Some(node);
        }

        node
    }

    // Opens a new non-terminal and adds it to the stack
    fn open_non_terminal_node(&mut self, kind: Nt) -> NodeIndex {
        let node = self.tree.add_node(CstNodeData::NonTerminal(kind));

        if let Some(parent) = self.node_stack.last_mut() {
            parent.children.push(CstNodeId(node));
        } else {
            // This is a root level node
            self.root_node_candidate = Some(node);
        }

        self.node_stack.push(NodeStackItem {
            node: CstNodeId(node),
            children: Vec::new(),
        });
        node
    }

    // Closes the current non-terminal
    fn close_non_terminal_node(&mut self) -> Option<CstNodeId> {
        let item = self.node_stack.pop();
        if let Some(item) = &item {
            let parent = item.node;
            item.children.iter().rev().for_each(|node| {
                self.tree.add_edge(parent.0, node.0, ());
            });
        }
        item.map(|item| item.node)
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

#[derive(Debug, Clone, Error)]
/// Error that occurs when constructing a view from a [NonTerminalHandle].
pub enum ViewConstructionError<T, Nt> {
    /// Expected a specific kind of terminal node, but got an invalid node
    #[error("Unexpected node for expected terminal: {terminal}")]
    UnexpectedTerminal {
        /// The index of the node.
        node: CstNodeId,
        /// The expected terminal.
        terminal: T,
    },
    /// Expected a specific kind of non-terminal node, but got an invalid node
    #[error("Unexpected node for expected non-terminal: {non_terminal}")]
    UnexpectedNonTerminal {
        /// The index of the node.
        node: CstNodeId,
        /// The expected non-terminal.
        non_terminal: Nt,
    },
    /// Expected an extra node, but got an invalid node
    #[error("Unexpected extra node")]
    UnexpectedExtraNode {
        /// The index of the node.
        node: CstNodeId,
    },
    /// Unexpected end of children
    #[error("Unexpected end of children")]
    UnexpectedEndOfChildren {
        /// The index of the node.
        parent: CstNodeId,
    },
    /// Unexpected empty children for a non-terminal
    #[error("Unexpected empty children for a non-terminal: {node}")]
    UnexpectedEmptyChildren {
        /// The index of the node.
        node: CstNodeId,
    },
    /// The node ID not found in the tree
    #[error("Node ID not found in the tree: {node}")]
    NodeIdNotFound {
        /// The index of the node.
        node: CstNodeId,
    },
}

pub fn assert_node_kind<T, Nt>(
    node: CstNodeId,
    kind: NodeKind<T, Nt>,
    expected_kind: NodeKind<T, Nt>,
) -> Result<(), ViewConstructionError<T, Nt>>
where
    T: PartialEq,
    Nt: PartialEq,
{
    if kind == expected_kind {
        Ok(())
    } else {
        match kind {
            NodeKind::Terminal(k) => {
                Err(ViewConstructionError::UnexpectedTerminal { node, terminal: k })
            }
            NodeKind::NonTerminal(k) => Err(ViewConstructionError::UnexpectedNonTerminal {
                node,
                non_terminal: k,
            }),
        }
    }
}

/// A trait that all generated non-terminal handles implements.
pub trait NonTerminalHandle<T, Nt> {
    /// The type of the view for this non-terminal.
    type View;
    /// Create a new non-terminal handle from a node.
    fn new(index: CstNodeId, kind: NodeKind<T, Nt>) -> Result<Self, ViewConstructionError<T, Nt>>
    where
        Self: Sized;

    /// Get the view of the non-terminal.
    fn get_view(
        &self,
        tree: &ConcreteSyntaxTree<T, Nt>,
    ) -> Result<Self::View, ViewConstructionError<T, Nt>>;
}
