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
    Cst, CstConstructError,
    ast::{BlockComment, LineComment, NewLine, RootHandle, Whitespace},
    common_visitors::{FormatVisitor, FormatVisitorError, InspectVisitor},
    nodes::{NonTerminalKind, TerminalKind},
    visitor::{BuiltinTerminalVisitor, CstHandleSuper as _},
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
    graph: DiGraph<Option<CstNodeData<T, Nt>>, ()>,
    dynamic_tokens: BTreeMap<DynamicTokenId, String>,
    root: CstNodeId,
}

impl<T, Nt> ConcreteSyntaxTree<T, Nt> {
    pub fn root(&self) -> CstNodeId {
        self.root
    }

    pub fn change_parent(&mut self, id: CstNodeId, parent: CstNodeId) {
        if let Some(edge) = self
            .graph
            .edges_directed(id.0, Direction::Incoming)
            .next()
            .map(|edge| edge.id())
        {
            self.graph.remove_edge(edge);
        }
        self.graph.add_edge(parent.0, id.0, ());
    }

    pub fn add_node(&mut self, data: CstNodeData<T, Nt>) -> CstNodeId {
        let node = self.graph.add_node(Some(data));
        CstNodeId(node)
    }

    pub fn add_node_with_parent(
        &mut self,
        data: CstNodeData<T, Nt>,
        parent: CstNodeId,
    ) -> CstNodeId {
        let node = self.add_node(data);
        self.graph.add_edge(parent.0, node.0, ());
        node
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

    pub fn update_node(
        &mut self,
        id: CstNodeId,
        data: CstNodeData<T, Nt>,
    ) -> Option<CstNodeData<T, Nt>> {
        if let Some(Some(node_data)) = self.graph.node_weight_mut(id.0) {
            Some(std::mem::replace(node_data, data))
        } else {
            None
        }
    }
}

impl<T, Nt> ConcreteSyntaxTree<T, Nt>
where
    T: Copy,
    Nt: Copy,
{
    pub fn node_data(&self, node: CstNodeId) -> Option<CstNodeData<T, Nt>> {
        self.graph.node_weight(node.0).copied().flatten()
    }

    /// Delete a node but keep its children
    pub fn delete_node(&mut self, id: CstNodeId) {
        // This does not remove the node from the graph, because it causes CstNodeId inconsistent.
        let edges = self
            .graph
            .edges_directed(id.0, Direction::Incoming)
            .map(|edge| edge.id())
            .collect::<Vec<_>>();
        for edge in edges {
            self.graph.remove_edge(edge);
        }
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
    pub fn collect_nodes<'v, const N: usize, V: BuiltinTerminalVisitor<E>, O, E>(
        &self,
        parent: CstNodeId,
        nodes: [NodeKind<TerminalKind, NonTerminalKind>; N],
        mut visitor: impl FnMut(
            [CstNodeId; N],
            &'v mut V,
        ) -> Result<(O, &'v mut V), CstConstructError<E>>,
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        let children = self.children(parent).collect::<Vec<_>>();
        let mut children = children.into_iter();
        let mut result = Vec::with_capacity(N);
        let mut ignored = Vec::with_capacity(N); // This N is just heuristic, we can have more than N ignored nodes
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
                                return Err(ViewConstructionError::UnexpectedTerminal {
                                    node: child,
                                    terminal: kind,
                                });
                            }
                            ignored.push((child, kind, data));
                            continue 'inner;
                        } else if kind.is_builtin_line_comment() || kind.is_builtin_block_comment()
                        {
                            ignored.push((child, kind, data));
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
        for (child, kind, data) in ignored {
            match kind {
                TerminalKind::Whitespace => visit_ignored.visit_builtin_whitespace_terminal(
                    Whitespace(child),
                    data,
                    self,
                )?,
                TerminalKind::NewLine => {
                    visit_ignored.visit_builtin_new_line_terminal(NewLine(child), data, self)?
                }
                TerminalKind::LineComment => visit_ignored.visit_builtin_line_comment_terminal(
                    LineComment(child),
                    data,
                    self,
                )?,
                TerminalKind::BlockComment => visit_ignored.visit_builtin_block_comment_terminal(
                    BlockComment(child),
                    data,
                    self,
                )?,
                _ => unreachable!(),
            }
        }
        let (result, visit_ignored) = visitor(
            result
                .try_into()
                .expect("Result should have the same length as nodes"),
            visit_ignored,
        )?;
        for child in children.by_ref() {
            let child_data = self
                .node_data(child)
                .ok_or(ViewConstructionError::NodeIdNotFound { node: child })?;
            match child_data {
                CstNodeData::Terminal { kind, data } => {
                    if kind.is_builtin_terminal() {
                        match kind {
                            TerminalKind::Whitespace => visit_ignored
                                .visit_builtin_whitespace_terminal(Whitespace(child), data, self)?,
                            TerminalKind::NewLine => visit_ignored
                                .visit_builtin_new_line_terminal(NewLine(child), data, self)?,
                            TerminalKind::LineComment => visit_ignored
                                .visit_builtin_line_comment_terminal(
                                    LineComment(child),
                                    data,
                                    self,
                                )?,
                            TerminalKind::BlockComment => visit_ignored
                                .visit_builtin_block_comment_terminal(
                                    BlockComment(child),
                                    data,
                                    self,
                                )?,
                            _ => unreachable!(),
                        }
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
        Ok(result)
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
pub struct CstBuilder {
    tree: DiGraph<Option<CstNodeData<TerminalKind, NonTerminalKind>>, ()>,
    node_stack: Vec<NodeStackItem>,
    root_node: Option<CstNodeId>,
}

#[derive(Debug, Clone)]
struct NodeStackItem {
    node: CstNodeId,
    span: InputSpan,
    children: Vec<CstNodeId>,
}

impl Default for CstBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl CstBuilder {
    pub fn new() -> Self {
        Self {
            tree: DiGraph::new(),
            node_stack: Vec::new(),
            root_node: None,
        }
    }

    // Adds a terminal node to the current non-terminal in the stack
    fn add_terminal_node(&mut self, kind: TerminalKind, span: InputSpan) -> NodeIndex {
        let node = self.tree.add_node(Some(CstNodeData::Terminal {
            kind,
            data: TerminalData::Input(span),
        }));

        let parent = self.node_stack.last_mut().expect("node stack is empty");
        parent.children.push(CstNodeId(node));
        parent.span = parent.span.merge(span);

        node
    }

    // Opens a new non-terminal and adds it to the stack
    fn open_non_terminal_node(&mut self, kind: NonTerminalKind) -> NodeIndex {
        // span will be filled later
        let node = self.tree.add_node(Some(CstNodeData::NonTerminal {
            kind,
            data: NonTerminalData::Input(InputSpan::EMPTY),
        }));

        if let Some(parent) = self.node_stack.last_mut() {
            parent.children.push(CstNodeId(node));
        } else {
            // This is a root level node
            self.root_node = Some(CstNodeId(node));
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
            let Some(CstNodeData::NonTerminal { data, .. }) = node_data else {
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
    pub fn build_tree(mut self) -> Cst {
        while !self.node_stack.is_empty() {
            self.close_non_terminal_node();
        }
        ConcreteSyntaxTree {
            graph: self.tree,
            dynamic_tokens: BTreeMap::new(),
            root: self.root_node.expect("Root node always provided"),
        }
    }
}

impl<'t> TreeConstruct<'t> for CstBuilder {
    type Error = ParolError;
    type Tree = Cst;

    fn open_non_terminal(
        &mut self,
        name: &'static str,
        _size_hint: Option<usize>,
    ) -> Result<(), Self::Error> {
        let kind = NonTerminalKind::from_non_terminal_name(name);
        self.open_non_terminal_node(kind);
        Ok(())
    }

    fn close_non_terminal(&mut self) -> Result<(), Self::Error> {
        self.close_non_terminal_node();
        Ok(())
    }

    fn add_token(&mut self, token: &Token<'t>) -> Result<(), Self::Error> {
        let kind = TerminalKind::from_terminal_index(token.token_type);
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

struct DummyTerminalVisitor;

impl<E> BuiltinTerminalVisitor<E> for DummyTerminalVisitor {
    fn visit_builtin_new_line_terminal(
        &mut self,
        _terminal: crate::ast::NewLine,
        _data: TerminalData,
        _tree: &crate::Cst,
    ) -> Result<(), E> {
        Ok(())
    }

    fn visit_builtin_whitespace_terminal(
        &mut self,
        _terminal: crate::ast::Whitespace,
        _data: TerminalData,
        _tree: &crate::Cst,
    ) -> Result<(), E> {
        Ok(())
    }

    fn visit_builtin_line_comment_terminal(
        &mut self,
        _terminal: crate::ast::LineComment,
        _data: TerminalData,
        _tree: &crate::Cst,
    ) -> Result<(), E> {
        Ok(())
    }

    fn visit_builtin_block_comment_terminal(
        &mut self,
        _terminal: crate::ast::BlockComment,
        _data: TerminalData,
        _tree: &crate::Cst,
    ) -> Result<(), E> {
        Ok(())
    }
}

/// A trait that all generated non-terminal handles implements.
pub trait NonTerminalHandle {
    /// The type of the view for this non-terminal.
    type View;

    fn node_id(&self) -> CstNodeId;

    /// Create a new non-terminal handle from a node.
    fn new(index: CstNodeId, tree: &Cst) -> Result<Self, CstConstructError>
    where
        Self: Sized,
    {
        Self::new_with_visit(index, tree, &mut DummyTerminalVisitor)
    }

    fn new_with_visit<E>(
        index: CstNodeId,
        tree: &Cst,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E>,
    ) -> Result<Self, CstConstructError<E>>
    where
        Self: Sized;

    /// Get the view of the non-terminal.
    fn get_view(&self, tree: &Cst) -> Result<Self::View, CstConstructError> {
        self.get_view_with_visit(
            tree,
            |view, visit_ignored| (view, visit_ignored),
            &mut DummyTerminalVisitor,
        )
    }

    /// Get the view of the non-terminal.
    fn get_view_with_visit<'v, V: BuiltinTerminalVisitor<E>, O, E>(
        &self,
        tree: &Cst,
        visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>>;

    /// Get the kind of the non-terminal.
    fn kind(&self) -> NonTerminalKind;
}

/// A trait that generated recursive views implements.
pub trait RecursiveView<T, Nt> {
    /// The type of the item in the view.
    type Item;
    fn get_all(
        &self,
        tree: &ConcreteSyntaxTree<T, Nt>,
    ) -> Result<Vec<Self::Item>, ViewConstructionError<T, Nt>> {
        self.get_all_with_visit(tree, &mut DummyTerminalVisitor)
    }

    fn get_all_with_visit<E>(
        &self,
        tree: &ConcreteSyntaxTree<T, Nt>,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E>,
    ) -> Result<Vec<Self::Item>, ViewConstructionError<T, Nt, E>>;
}
