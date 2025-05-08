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
use std::{collections::BTreeMap, convert::Infallible};
use thiserror::Error;

pub use span::*;

use crate::{
    CstConstructError,
    ast::RootHandle,
    common_visitors::{FormatVisitor, FormatVisitorError, InspectVisitor},
    nodes::{NonTerminalKind, TerminalKind},
    visitor::CstHandleSuper as _,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
/// A dynamic token id that provided by the user land.
pub struct DynamicTokenId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CstNodeData<T, Nt> {
    /// A terminal node with its kind and span
    Terminal { kind: T, data: TerminalData },
    /// A non-terminal node with its kind
    NonTerminal { kind: Nt, data: NonTerminalData },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TerminalData {
    Input(InputSpan),
    Dynamic(DynamicTokenId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NonTerminalData {
    Input(InputSpan),
    Dynamic,
}

impl<T, Nt> CstNodeData<T, Nt>
where
    T: Copy,
    Nt: Copy,
{
    pub fn node_kind(&self) -> NodeKind<T, Nt> {
        match self {
            CstNodeData::Terminal { kind, .. } => NodeKind::Terminal(*kind),
            CstNodeData::NonTerminal { kind, .. } => NodeKind::NonTerminal(*kind),
        }
    }
}

impl<T, Nt> CstNodeData<T, Nt>
where
    T: PartialEq + Copy,
    Nt: PartialEq + Copy,
{
    pub fn expected_terminal_or_error(
        &self,
        node: CstNodeId,
        expected: T,
    ) -> Result<(T, TerminalData), ViewConstructionError<T, Nt>> {
        match self {
            CstNodeData::Terminal { kind, data } if *kind == expected => Ok((*kind, *data)),
            CstNodeData::Terminal { kind, .. } => Err(ViewConstructionError::UnexpectedTerminal {
                node,
                terminal: *kind,
            }),
            CstNodeData::NonTerminal { kind, .. } => {
                Err(ViewConstructionError::UnexpectedNonTerminal {
                    node,
                    non_terminal: *kind,
                })
            }
        }
    }

    pub fn expected_non_terminal_or_error(
        &self,
        node: CstNodeId,
        expected: Nt,
    ) -> Result<(Nt, NonTerminalData), ViewConstructionError<T, Nt>> {
        match self {
            CstNodeData::NonTerminal { kind, data } if *kind == expected => Ok((*kind, *data)),
            CstNodeData::NonTerminal { kind, .. } => {
                Err(ViewConstructionError::UnexpectedNonTerminal {
                    node,
                    non_terminal: *kind,
                })
            }
            CstNodeData::Terminal { kind, .. } => Err(ViewConstructionError::UnexpectedTerminal {
                node,
                terminal: *kind,
            }),
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

    pub fn dynamic_token(&self, id: DynamicTokenId) -> Option<&str> {
        self.dynamic_tokens.get(&id).map(|s| s.as_str())
    }
}

impl<T, Nt> ConcreteSyntaxTree<T, Nt>
where
    T: Copy,
    Nt: Copy,
{
    pub fn node_data(&self, node: CstNodeId) -> Option<CstNodeData<T, Nt>> {
        self.graph.node_weight(node.0).copied()
    }
}

impl TerminalKind {
    fn auto_ws_is_off(&self, index: usize) -> bool {
        // TODO: support CodeBlockDelimitor it's also used for beginning of CodeBlock and it's non-terminal so need more analysis to handle this case
        matches!(
            (self, index),
            (
                TerminalKind::Ws
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
    pub fn collect_nodes<const N: usize, E>(
        &self,
        parent: CstNodeId,
        nodes: [NodeKind<TerminalKind, NonTerminalKind>; N],
        mut visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<[CstNodeId; N], CstConstructError<E>> {
        let mut children = self.children(parent);
        let mut result = Vec::with_capacity(N);
        'outer: for expected_kind in nodes {
            'inner: for (idx, child) in children.by_ref().enumerate() {
                let child_data = self
                    .node_data(child)
                    .ok_or(ViewConstructionError::NodeIdNotFound { node: child })?;
                match child_data {
                    CstNodeData::Terminal { kind, data } => {
                        if NodeKind::Terminal(kind) == expected_kind {
                            result.push(child);
                            continue 'outer;
                        } else if kind.is_builtin_whitespace() || kind.is_builtin_new_line() {
                            if kind.auto_ws_is_off(idx) {
                                println!("auto_ws_is_off: {:?}", kind);
                                return Err(ViewConstructionError::UnexpectedTerminal {
                                    node: child,
                                    terminal: kind,
                                });
                            }
                            visit_ignored(child, kind, data)?;
                            continue 'inner;
                        } else if kind.is_builtin_line_comment() || kind.is_builtin_block_comment()
                        {
                            // comments
                            visit_ignored(child, kind, data)?;
                            continue 'inner;
                        } else {
                            return Err(ViewConstructionError::UnexpectedTerminal {
                                node: child,
                                terminal: kind,
                            });
                        }
                    }
                    CstNodeData::NonTerminal { kind, .. } => {
                        if NodeKind::NonTerminal(kind) == expected_kind {
                            result.push(child);
                            continue 'outer;
                        } else {
                            return Err(ViewConstructionError::UnexpectedNonTerminal {
                                node: child,
                                non_terminal: kind,
                            });
                        }
                    }
                }
            }
            return Err(ViewConstructionError::UnexpectedEndOfChildren { parent });
        }
        for child in children.by_ref() {
            let child_data = self
                .node_data(child)
                .ok_or(ViewConstructionError::NodeIdNotFound { node: child })?;
            match child_data {
                CstNodeData::Terminal { kind, data } => {
                    if kind.is_builtin_terminal() {
                        visit_ignored(child, kind, data)?;
                    } else {
                        return Err(ViewConstructionError::UnexpectedTerminal {
                            node: child,
                            terminal: kind,
                        });
                    }
                }
                CstNodeData::NonTerminal { kind, .. } => {
                    return Err(ViewConstructionError::UnexpectedNonTerminal {
                        node: child,
                        non_terminal: kind,
                    });
                }
            }
        }
        Ok(result
            .try_into()
            .expect("Result should have the same length as nodes"))
    }

    pub fn root_handle(&self) -> RootHandle {
        RootHandle(self.root())
    }

    pub fn write(
        &self,
        input: &str,
        w: &mut impl std::fmt::Write,
    ) -> Result<(), FormatVisitorError> {
        let mut visitor = FormatVisitor::new(input, w);
        visitor.visit_root_handle(self.root_handle(), self)?;
        Ok(())
    }

    pub fn inspect(
        &self,
        input: &str,
        w: &mut impl std::fmt::Write,
    ) -> Result<(), FormatVisitorError> {
        let mut visitor = InspectVisitor::new(input, w);
        visitor.visit_root_handle(self.root_handle(), self)?;
        Ok(())
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
    span: InputSpan,
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
        let node = self.tree.add_node(CstNodeData::Terminal {
            kind,
            data: TerminalData::Input(span),
        });

        if let Some(parent) = self.node_stack.last_mut() {
            parent.children.push(CstNodeId(node));
            parent.span = parent.span.merge(span);
        } else {
            // This is a root level node
            self.root_node_candidate = Some(node);
        }

        node
    }

    // Opens a new non-terminal and adds it to the stack
    fn open_non_terminal_node(&mut self, kind: Nt) -> NodeIndex {
        // span will be filled later
        let node = self.tree.add_node(CstNodeData::NonTerminal {
            kind,
            data: NonTerminalData::Input(InputSpan::EMPTY),
        });

        if let Some(parent) = self.node_stack.last_mut() {
            parent.children.push(CstNodeId(node));
        } else {
            // This is a root level node
            self.root_node_candidate = Some(node);
        }

        self.node_stack.push(NodeStackItem {
            node: CstNodeId(node),
            span: InputSpan::EMPTY,
            children: Vec::new(),
        });
        node
    }

    // Closes the current non-terminal
    fn close_non_terminal_node(&mut self) -> Option<CstNodeId> {
        let popped = self.node_stack.pop();
        if let Some(item) = &popped {
            let parent = item.node;
            item.children.iter().rev().for_each(|node| {
                self.tree.add_edge(parent.0, node.0, ());
            });
            let node_data = self
                .tree
                .node_weight_mut(parent.0)
                .expect("this node must be created on open_non_terminal_node");
            let CstNodeData::NonTerminal { data, .. } = node_data else {
                panic!("this node must be created as NonTerminal");
            };
            let NonTerminalData::Input(span) = data else {
                panic!("this node must be created as input node");
            };
            *span = item.span;
            if let Some(parent) = self.node_stack.last_mut() {
                parent.span = parent.span.merge(item.span);
            }
        }
        popped.map(|item| item.node)
    }

    /// Builds the tree
    pub fn build_tree(mut self) -> ConcreteSyntaxTree<T, Nt> {
        while !self.node_stack.is_empty() {
            self.close_non_terminal_node();
        }
        ConcreteSyntaxTree {
            graph: self.tree,
            dynamic_tokens: BTreeMap::new(),
            root: CstNodeId(self.root_node_candidate.expect("Root node always provided")),
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
        Ok(self.build_tree())
    }
}

#[derive(Debug, Clone, Error)]
/// Error that occurs when constructing a view from a [NonTerminalHandle].
pub enum ViewConstructionError<T, Nt, E = Infallible> {
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
    /// Error that occurs when constructing a view from a [NonTerminalHandle].
    #[error(transparent)]
    Error(#[from] E),
}

impl<T, Nt, E> ViewConstructionError<T, Nt, E> {
    pub fn extract_error(self) -> Result<E, ViewConstructionError<T, Nt, Infallible>> {
        match self {
            ViewConstructionError::Error(e) => Ok(e),
            ViewConstructionError::UnexpectedTerminal { node, terminal } => {
                Err(ViewConstructionError::UnexpectedTerminal { node, terminal })
            }
            ViewConstructionError::UnexpectedNonTerminal { node, non_terminal } => {
                Err(ViewConstructionError::UnexpectedNonTerminal { node, non_terminal })
            }
            ViewConstructionError::UnexpectedExtraNode { node } => {
                Err(ViewConstructionError::UnexpectedExtraNode { node })
            }
            ViewConstructionError::UnexpectedEndOfChildren { parent } => {
                Err(ViewConstructionError::UnexpectedEndOfChildren { parent })
            }
            ViewConstructionError::UnexpectedEmptyChildren { node } => {
                Err(ViewConstructionError::UnexpectedEmptyChildren { node })
            }
            ViewConstructionError::NodeIdNotFound { node } => {
                Err(ViewConstructionError::NodeIdNotFound { node })
            }
        }
    }
}

impl<T, Nt> ViewConstructionError<T, Nt, Infallible> {
    pub fn into_any_error<E>(self) -> ViewConstructionError<T, Nt, E> {
        match self {
            ViewConstructionError::UnexpectedTerminal { node, terminal } => {
                ViewConstructionError::UnexpectedTerminal { node, terminal }
            }
            ViewConstructionError::UnexpectedNonTerminal { node, non_terminal } => {
                ViewConstructionError::UnexpectedNonTerminal { node, non_terminal }
            }
            ViewConstructionError::UnexpectedExtraNode { node } => {
                ViewConstructionError::UnexpectedExtraNode { node }
            }
            ViewConstructionError::UnexpectedEndOfChildren { parent } => {
                ViewConstructionError::UnexpectedEndOfChildren { parent }
            }
            ViewConstructionError::UnexpectedEmptyChildren { node } => {
                ViewConstructionError::UnexpectedEmptyChildren { node }
            }
            ViewConstructionError::NodeIdNotFound { node } => {
                ViewConstructionError::NodeIdNotFound { node }
            }
        }
    }
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
    ) -> Result<Self::View, ViewConstructionError<T, Nt>> {
        self.get_view_with_visit(tree, |_, _, _| Ok(()))
    }

    /// Get the view of the non-terminal.
    fn get_view_with_visit<E>(
        &self,
        tree: &ConcreteSyntaxTree<T, Nt>,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, ViewConstructionError<T, Nt, E>>;

    /// Get the kind of the non-terminal.
    fn kind(&self) -> Nt;
}
