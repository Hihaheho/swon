#![allow(unused_variables)]
use super::tree::{
    NonTerminalHandle, TerminalData, CstNodeId, ViewConstructionError, assert_node_kind,
};
use crate::{Cst, CstConstructError};
use super::nodes::{TerminalKind, NonTerminalKind};
use parol_runtime::parser::parse_tree_type::NodeKind;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArrayHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for ArrayHandle {
    type View = ArrayView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Array))?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Array
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [array_begin, array_list, array_end] = tree
            .collect_nodes(
                self.0,
                [
                    NodeKind::NonTerminal(NonTerminalKind::ArrayBegin),
                    NodeKind::NonTerminal(NonTerminalKind::ArrayList),
                    NodeKind::NonTerminal(NonTerminalKind::ArrayEnd),
                ],
                visit_ignored,
            )?;
        Ok(ArrayView {
            array_begin: ArrayBeginHandle(array_begin),
            array_list: ArrayListHandle(array_list),
            array_end: ArrayEndHandle(array_end),
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArrayView {
    pub array_begin: ArrayBeginHandle,
    pub array_list: ArrayListHandle,
    pub array_end: ArrayEndHandle,
}
impl ArrayView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArrayBeginHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for ArrayBeginHandle {
    type View = ArrayBeginView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(
            index,
            kind,
            NodeKind::NonTerminal(NonTerminalKind::ArrayBegin),
        )?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::ArrayBegin
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [l_bracket] = tree
            .collect_nodes(
                self.0,
                [NodeKind::Terminal(TerminalKind::LBracket)],
                visit_ignored,
            )?;
        Ok(ArrayBeginView {
            l_bracket: LBracket(l_bracket),
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArrayBeginView {
    pub l_bracket: LBracket,
}
impl ArrayBeginView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArrayEndHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for ArrayEndHandle {
    type View = ArrayEndView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::ArrayEnd))?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::ArrayEnd
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [r_bracket] = tree
            .collect_nodes(
                self.0,
                [NodeKind::Terminal(TerminalKind::RBracket)],
                visit_ignored,
            )?;
        Ok(ArrayEndView {
            r_bracket: RBracket(r_bracket),
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArrayEndView {
    pub r_bracket: RBracket,
}
impl ArrayEndView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArrayListHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for ArrayListHandle {
    type View = Option<ArrayListView>;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(
            index,
            kind,
            NodeKind::NonTerminal(NonTerminalKind::ArrayList),
        )?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::ArrayList
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        if tree.has_no_children(self.0) {
            return Ok(None);
        }
        let [value, array_opt, array_list] = tree
            .collect_nodes(
                self.0,
                [
                    NodeKind::NonTerminal(NonTerminalKind::Value),
                    NodeKind::NonTerminal(NonTerminalKind::ArrayOpt),
                    NodeKind::NonTerminal(NonTerminalKind::ArrayList),
                ],
                visit_ignored,
            )?;
        Ok(
            Some(ArrayListView {
                value: ValueHandle(value),
                array_opt: ArrayOptHandle(array_opt),
                array_list: ArrayListHandle(array_list),
            }),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArrayListView {
    pub value: ValueHandle,
    pub array_opt: ArrayOptHandle,
    pub array_list: ArrayListHandle,
}
impl ArrayListView {
    pub fn get_all(&self, tree: &Cst) -> Result<Vec<ArrayListItem>, CstConstructError> {
        self.get_all_with_visit(tree, |_, _, _| Ok(()))
    }
    pub fn get_all_with_visit<E>(
        &self,
        tree: &Cst,
        mut visit_ignored: impl FnMut(
            CstNodeId,
            TerminalKind,
            TerminalData,
        ) -> Result<(), E>,
    ) -> Result<Vec<ArrayListItem>, CstConstructError<E>> {
        let mut items = Vec::new();
        let mut current_view = Some(*self);
        while let Some(item) = current_view {
            let Self { value, array_opt, .. } = item;
            items.push(ArrayListItem { value, array_opt });
            current_view = item
                .array_list
                .get_view_with_visit(tree, &mut visit_ignored)?;
        }
        Ok(items)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArrayListItem {
    pub value: ValueHandle,
    pub array_opt: ArrayOptHandle,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArrayMarkerHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for ArrayMarkerHandle {
    type View = ArrayMarkerView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(
            index,
            kind,
            NodeKind::NonTerminal(NonTerminalKind::ArrayMarker),
        )?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::ArrayMarker
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [array_begin, array_marker_opt, array_end] = tree
            .collect_nodes(
                self.0,
                [
                    NodeKind::NonTerminal(NonTerminalKind::ArrayBegin),
                    NodeKind::NonTerminal(NonTerminalKind::ArrayMarkerOpt),
                    NodeKind::NonTerminal(NonTerminalKind::ArrayEnd),
                ],
                visit_ignored,
            )?;
        Ok(ArrayMarkerView {
            array_begin: ArrayBeginHandle(array_begin),
            array_marker_opt: ArrayMarkerOptHandle(array_marker_opt),
            array_end: ArrayEndHandle(array_end),
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArrayMarkerView {
    pub array_begin: ArrayBeginHandle,
    pub array_marker_opt: ArrayMarkerOptHandle,
    pub array_end: ArrayEndHandle,
}
impl ArrayMarkerView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArrayMarkerOptHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for ArrayMarkerOptHandle {
    type View = Option<IntegerHandle>;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(
            index,
            kind,
            NodeKind::NonTerminal(NonTerminalKind::ArrayMarkerOpt),
        )?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::ArrayMarkerOpt
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        if tree.has_no_children(self.0) {
            return Ok(None);
        }
        let [child] = tree
            .collect_nodes(
                self.0,
                [NodeKind::NonTerminal(NonTerminalKind::Integer)],
                visit_ignored,
            )?;
        Ok(
            Some(
                IntegerHandle::new(
                        child,
                        NodeKind::NonTerminal(NonTerminalKind::Integer),
                    )
                    .map_err(|e| e.into_any_error::<E>())?,
            ),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArrayOptHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for ArrayOptHandle {
    type View = Option<CommaHandle>;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::ArrayOpt))?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::ArrayOpt
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        if tree.has_no_children(self.0) {
            return Ok(None);
        }
        let [child] = tree
            .collect_nodes(
                self.0,
                [NodeKind::NonTerminal(NonTerminalKind::Comma)],
                visit_ignored,
            )?;
        Ok(
            Some(
                CommaHandle::new(child, NodeKind::NonTerminal(NonTerminalKind::Comma))
                    .map_err(|e| e.into_any_error::<E>())?,
            ),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AtHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for AtHandle {
    type View = AtView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::At))?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::At
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [at] = tree
            .collect_nodes(
                self.0,
                [NodeKind::Terminal(TerminalKind::At)],
                visit_ignored,
            )?;
        Ok(AtView { at: At(at) })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AtView {
    pub at: At,
}
impl AtView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BeginHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for BeginHandle {
    type View = BeginView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Begin))?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Begin
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [l_brace] = tree
            .collect_nodes(
                self.0,
                [NodeKind::Terminal(TerminalKind::LBrace)],
                visit_ignored,
            )?;
        Ok(BeginView {
            l_brace: LBrace(l_brace),
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BeginView {
    pub l_brace: LBrace,
}
impl BeginView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BindHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for BindHandle {
    type View = BindView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Bind))?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Bind
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [bind] = tree
            .collect_nodes(
                self.0,
                [NodeKind::Terminal(TerminalKind::Bind)],
                visit_ignored,
            )?;
        Ok(BindView { bind: Bind(bind) })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BindView {
    pub bind: Bind,
}
impl BindView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BindingHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for BindingHandle {
    type View = BindingView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Binding))?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Binding
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [keys, binding_rhs] = tree
            .collect_nodes(
                self.0,
                [
                    NodeKind::NonTerminal(NonTerminalKind::Keys),
                    NodeKind::NonTerminal(NonTerminalKind::BindingRhs),
                ],
                visit_ignored,
            )?;
        Ok(BindingView {
            keys: KeysHandle(keys),
            binding_rhs: BindingRhsHandle(binding_rhs),
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BindingView {
    pub keys: KeysHandle,
    pub binding_rhs: BindingRhsHandle,
}
impl BindingView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BindingRhsHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for BindingRhsHandle {
    type View = BindingRhsView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(
            index,
            kind,
            NodeKind::NonTerminal(NonTerminalKind::BindingRhs),
        )?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::BindingRhs
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let mut children = tree.children(self.0);
        let Some(child) = children.next() else {
            return Err(ViewConstructionError::UnexpectedEndOfChildren {
                parent: self.0,
            });
        };
        let Some(child_data) = tree.node_data(child) else {
            return Err(ViewConstructionError::NodeIdNotFound {
                node: child,
            });
        };
        let variant = match child_data.node_kind() {
            NodeKind::NonTerminal(NonTerminalKind::ValueBinding) => {
                BindingRhsView::ValueBinding(ValueBindingHandle(child))
            }
            NodeKind::NonTerminal(NonTerminalKind::SectionBinding) => {
                BindingRhsView::SectionBinding(SectionBindingHandle(child))
            }
            NodeKind::NonTerminal(NonTerminalKind::TextBinding) => {
                BindingRhsView::TextBinding(TextBindingHandle(child))
            }
            NodeKind::Terminal(kind) => {
                return Err(ViewConstructionError::UnexpectedTerminal {
                    node: child,
                    terminal: kind,
                });
            }
            NodeKind::NonTerminal(kind) => {
                return Err(ViewConstructionError::UnexpectedNonTerminal {
                    node: child,
                    non_terminal: kind,
                });
            }
        };
        if let Some(child) = children.next() {
            return Err(ViewConstructionError::UnexpectedExtraNode {
                node: child,
            });
        }
        Ok(variant)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindingRhsView {
    ValueBinding(ValueBindingHandle),
    SectionBinding(SectionBindingHandle),
    TextBinding(TextBindingHandle),
}
impl BindingRhsView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BooleanHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for BooleanHandle {
    type View = BooleanView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Boolean))?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Boolean
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let mut children = tree.children(self.0);
        let Some(child) = children.next() else {
            return Err(ViewConstructionError::UnexpectedEndOfChildren {
                parent: self.0,
            });
        };
        let Some(child_data) = tree.node_data(child) else {
            return Err(ViewConstructionError::NodeIdNotFound {
                node: child,
            });
        };
        let variant = match child_data.node_kind() {
            NodeKind::NonTerminal(NonTerminalKind::True) => {
                BooleanView::True(TrueHandle(child))
            }
            NodeKind::NonTerminal(NonTerminalKind::False) => {
                BooleanView::False(FalseHandle(child))
            }
            NodeKind::Terminal(kind) => {
                return Err(ViewConstructionError::UnexpectedTerminal {
                    node: child,
                    terminal: kind,
                });
            }
            NodeKind::NonTerminal(kind) => {
                return Err(ViewConstructionError::UnexpectedNonTerminal {
                    node: child,
                    non_terminal: kind,
                });
            }
        };
        if let Some(child) = children.next() {
            return Err(ViewConstructionError::UnexpectedExtraNode {
                node: child,
            });
        }
        Ok(variant)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BooleanView {
    True(TrueHandle),
    False(FalseHandle),
}
impl BooleanView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CodeHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for CodeHandle {
    type View = CodeView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Code))?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Code
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [code] = tree
            .collect_nodes(
                self.0,
                [NodeKind::Terminal(TerminalKind::Code)],
                visit_ignored,
            )?;
        Ok(CodeView { code: Code(code) })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodeView {
    pub code: Code,
}
impl CodeView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CodeBlockHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for CodeBlockHandle {
    type View = CodeBlockView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(
            index,
            kind,
            NodeKind::NonTerminal(NonTerminalKind::CodeBlock),
        )?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::CodeBlock
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [code_block_delimiter, code_block_tail_common] = tree
            .collect_nodes(
                self.0,
                [
                    NodeKind::NonTerminal(NonTerminalKind::CodeBlockDelimiter),
                    NodeKind::NonTerminal(NonTerminalKind::CodeBlockTailCommon),
                ],
                visit_ignored,
            )?;
        Ok(CodeBlockView {
            code_block_delimiter: CodeBlockDelimiterHandle(code_block_delimiter),
            code_block_tail_common: CodeBlockTailCommonHandle(code_block_tail_common),
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodeBlockView {
    pub code_block_delimiter: CodeBlockDelimiterHandle,
    pub code_block_tail_common: CodeBlockTailCommonHandle,
}
impl CodeBlockView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CodeBlockDelimiterHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for CodeBlockDelimiterHandle {
    type View = CodeBlockDelimiterView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(
            index,
            kind,
            NodeKind::NonTerminal(NonTerminalKind::CodeBlockDelimiter),
        )?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::CodeBlockDelimiter
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [code_block_delimiter] = tree
            .collect_nodes(
                self.0,
                [NodeKind::Terminal(TerminalKind::CodeBlockDelimiter)],
                visit_ignored,
            )?;
        Ok(CodeBlockDelimiterView {
            code_block_delimiter: CodeBlockDelimiter(code_block_delimiter),
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodeBlockDelimiterView {
    pub code_block_delimiter: CodeBlockDelimiter,
}
impl CodeBlockDelimiterView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CodeBlockLineHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for CodeBlockLineHandle {
    type View = CodeBlockLineView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(
            index,
            kind,
            NodeKind::NonTerminal(NonTerminalKind::CodeBlockLine),
        )?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::CodeBlockLine
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [code_block_line] = tree
            .collect_nodes(
                self.0,
                [NodeKind::Terminal(TerminalKind::CodeBlockLine)],
                visit_ignored,
            )?;
        Ok(CodeBlockLineView {
            code_block_line: CodeBlockLine(code_block_line),
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodeBlockLineView {
    pub code_block_line: CodeBlockLine,
}
impl CodeBlockLineView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CodeBlockTailCommonHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for CodeBlockTailCommonHandle {
    type View = CodeBlockTailCommonView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(
            index,
            kind,
            NodeKind::NonTerminal(NonTerminalKind::CodeBlockTailCommon),
        )?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::CodeBlockTailCommon
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [newline, code_block_tail_common_list, code_block_tail_common_opt,
        code_block_delimiter] = tree
            .collect_nodes(
                self.0,
                [
                    NodeKind::NonTerminal(NonTerminalKind::Newline),
                    NodeKind::NonTerminal(NonTerminalKind::CodeBlockTailCommonList),
                    NodeKind::NonTerminal(NonTerminalKind::CodeBlockTailCommonOpt),
                    NodeKind::NonTerminal(NonTerminalKind::CodeBlockDelimiter),
                ],
                visit_ignored,
            )?;
        Ok(CodeBlockTailCommonView {
            newline: NewlineHandle(newline),
            code_block_tail_common_list: CodeBlockTailCommonListHandle(
                code_block_tail_common_list,
            ),
            code_block_tail_common_opt: CodeBlockTailCommonOptHandle(
                code_block_tail_common_opt,
            ),
            code_block_delimiter: CodeBlockDelimiterHandle(code_block_delimiter),
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodeBlockTailCommonView {
    pub newline: NewlineHandle,
    pub code_block_tail_common_list: CodeBlockTailCommonListHandle,
    pub code_block_tail_common_opt: CodeBlockTailCommonOptHandle,
    pub code_block_delimiter: CodeBlockDelimiterHandle,
}
impl CodeBlockTailCommonView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CodeBlockTailCommonListHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for CodeBlockTailCommonListHandle {
    type View = Option<CodeBlockTailCommonListView>;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(
            index,
            kind,
            NodeKind::NonTerminal(NonTerminalKind::CodeBlockTailCommonList),
        )?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::CodeBlockTailCommonList
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        if tree.has_no_children(self.0) {
            return Ok(None);
        }
        let [code_block_line, newline, code_block_tail_common_list] = tree
            .collect_nodes(
                self.0,
                [
                    NodeKind::NonTerminal(NonTerminalKind::CodeBlockLine),
                    NodeKind::NonTerminal(NonTerminalKind::Newline),
                    NodeKind::NonTerminal(NonTerminalKind::CodeBlockTailCommonList),
                ],
                visit_ignored,
            )?;
        Ok(
            Some(CodeBlockTailCommonListView {
                code_block_line: CodeBlockLineHandle(code_block_line),
                newline: NewlineHandle(newline),
                code_block_tail_common_list: CodeBlockTailCommonListHandle(
                    code_block_tail_common_list,
                ),
            }),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodeBlockTailCommonListView {
    pub code_block_line: CodeBlockLineHandle,
    pub newline: NewlineHandle,
    pub code_block_tail_common_list: CodeBlockTailCommonListHandle,
}
impl CodeBlockTailCommonListView {
    pub fn get_all(
        &self,
        tree: &Cst,
    ) -> Result<Vec<CodeBlockTailCommonListItem>, CstConstructError> {
        self.get_all_with_visit(tree, |_, _, _| Ok(()))
    }
    pub fn get_all_with_visit<E>(
        &self,
        tree: &Cst,
        mut visit_ignored: impl FnMut(
            CstNodeId,
            TerminalKind,
            TerminalData,
        ) -> Result<(), E>,
    ) -> Result<Vec<CodeBlockTailCommonListItem>, CstConstructError<E>> {
        let mut items = Vec::new();
        let mut current_view = Some(*self);
        while let Some(item) = current_view {
            let Self { code_block_line, newline, .. } = item;
            items
                .push(CodeBlockTailCommonListItem {
                    code_block_line,
                    newline,
                });
            current_view = item
                .code_block_tail_common_list
                .get_view_with_visit(tree, &mut visit_ignored)?;
        }
        Ok(items)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodeBlockTailCommonListItem {
    pub code_block_line: CodeBlockLineHandle,
    pub newline: NewlineHandle,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CodeBlockTailCommonOptHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for CodeBlockTailCommonOptHandle {
    type View = Option<WsHandle>;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(
            index,
            kind,
            NodeKind::NonTerminal(NonTerminalKind::CodeBlockTailCommonOpt),
        )?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::CodeBlockTailCommonOpt
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        if tree.has_no_children(self.0) {
            return Ok(None);
        }
        let [child] = tree
            .collect_nodes(
                self.0,
                [NodeKind::NonTerminal(NonTerminalKind::Ws)],
                visit_ignored,
            )?;
        Ok(
            Some(
                WsHandle::new(child, NodeKind::NonTerminal(NonTerminalKind::Ws))
                    .map_err(|e| e.into_any_error::<E>())?,
            ),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommaHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for CommaHandle {
    type View = CommaView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Comma))?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Comma
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [comma] = tree
            .collect_nodes(
                self.0,
                [NodeKind::Terminal(TerminalKind::Comma)],
                visit_ignored,
            )?;
        Ok(CommaView { comma: Comma(comma) })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommaView {
    pub comma: Comma,
}
impl CommaView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ContinueHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for ContinueHandle {
    type View = ContinueView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Continue))?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Continue
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [esc] = tree
            .collect_nodes(
                self.0,
                [NodeKind::Terminal(TerminalKind::Esc)],
                visit_ignored,
            )?;
        Ok(ContinueView { esc: Esc(esc) })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContinueView {
    pub esc: Esc,
}
impl ContinueView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DotHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for DotHandle {
    type View = DotView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Dot))?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Dot
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [dot] = tree
            .collect_nodes(
                self.0,
                [NodeKind::Terminal(TerminalKind::Dot)],
                visit_ignored,
            )?;
        Ok(DotView { dot: Dot(dot) })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DotView {
    pub dot: Dot,
}
impl DotView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EndHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for EndHandle {
    type View = EndView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::End))?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::End
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [r_brace] = tree
            .collect_nodes(
                self.0,
                [NodeKind::Terminal(TerminalKind::RBrace)],
                visit_ignored,
            )?;
        Ok(EndView {
            r_brace: RBrace(r_brace),
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EndView {
    pub r_brace: RBrace,
}
impl EndView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExtHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for ExtHandle {
    type View = ExtView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Ext))?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Ext
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [dollar] = tree
            .collect_nodes(
                self.0,
                [NodeKind::Terminal(TerminalKind::Dollar)],
                visit_ignored,
            )?;
        Ok(ExtView { dollar: Dollar(dollar) })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtView {
    pub dollar: Dollar,
}
impl ExtView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExtensionNameSpaceHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for ExtensionNameSpaceHandle {
    type View = ExtensionNameSpaceView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(
            index,
            kind,
            NodeKind::NonTerminal(NonTerminalKind::ExtensionNameSpace),
        )?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::ExtensionNameSpace
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [ext, ident] = tree
            .collect_nodes(
                self.0,
                [
                    NodeKind::NonTerminal(NonTerminalKind::Ext),
                    NodeKind::NonTerminal(NonTerminalKind::Ident),
                ],
                visit_ignored,
            )?;
        Ok(ExtensionNameSpaceView {
            ext: ExtHandle(ext),
            ident: IdentHandle(ident),
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtensionNameSpaceView {
    pub ext: ExtHandle,
    pub ident: IdentHandle,
}
impl ExtensionNameSpaceView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FalseHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for FalseHandle {
    type View = FalseView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::False))?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::False
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [r#false] = tree
            .collect_nodes(
                self.0,
                [NodeKind::Terminal(TerminalKind::False)],
                visit_ignored,
            )?;
        Ok(FalseView {
            r#false: False(r#false),
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FalseView {
    pub r#false: False,
}
impl FalseView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HoleHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for HoleHandle {
    type View = HoleView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Hole))?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Hole
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [hole] = tree
            .collect_nodes(
                self.0,
                [NodeKind::Terminal(TerminalKind::Hole)],
                visit_ignored,
            )?;
        Ok(HoleView { hole: Hole(hole) })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HoleView {
    pub hole: Hole,
}
impl HoleView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IdentHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for IdentHandle {
    type View = IdentView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Ident))?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Ident
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [ident] = tree
            .collect_nodes(
                self.0,
                [NodeKind::Terminal(TerminalKind::Ident)],
                visit_ignored,
            )?;
        Ok(IdentView { ident: Ident(ident) })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IdentView {
    pub ident: Ident,
}
impl IdentView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InStrHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for InStrHandle {
    type View = InStrView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::InStr))?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::InStr
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [in_str] = tree
            .collect_nodes(
                self.0,
                [NodeKind::Terminal(TerminalKind::InStr)],
                visit_ignored,
            )?;
        Ok(InStrView { in_str: InStr(in_str) })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InStrView {
    pub in_str: InStr,
}
impl InStrView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IntegerHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for IntegerHandle {
    type View = IntegerView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Integer))?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Integer
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [integer] = tree
            .collect_nodes(
                self.0,
                [NodeKind::Terminal(TerminalKind::Integer)],
                visit_ignored,
            )?;
        Ok(IntegerView {
            integer: Integer(integer),
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntegerView {
    pub integer: Integer,
}
impl IntegerView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeyHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for KeyHandle {
    type View = KeyView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Key))?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Key
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [key_base, key_opt] = tree
            .collect_nodes(
                self.0,
                [
                    NodeKind::NonTerminal(NonTerminalKind::KeyBase),
                    NodeKind::NonTerminal(NonTerminalKind::KeyOpt),
                ],
                visit_ignored,
            )?;
        Ok(KeyView {
            key_base: KeyBaseHandle(key_base),
            key_opt: KeyOptHandle(key_opt),
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyView {
    pub key_base: KeyBaseHandle,
    pub key_opt: KeyOptHandle,
}
impl KeyView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeyBaseHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for KeyBaseHandle {
    type View = KeyBaseView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::KeyBase))?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::KeyBase
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let mut children = tree.children(self.0);
        let Some(child) = children.next() else {
            return Err(ViewConstructionError::UnexpectedEndOfChildren {
                parent: self.0,
            });
        };
        let Some(child_data) = tree.node_data(child) else {
            return Err(ViewConstructionError::NodeIdNotFound {
                node: child,
            });
        };
        let variant = match child_data.node_kind() {
            NodeKind::NonTerminal(NonTerminalKind::Ident) => {
                KeyBaseView::Ident(IdentHandle(child))
            }
            NodeKind::NonTerminal(NonTerminalKind::ExtensionNameSpace) => {
                KeyBaseView::ExtensionNameSpace(ExtensionNameSpaceHandle(child))
            }
            NodeKind::NonTerminal(NonTerminalKind::Str) => {
                KeyBaseView::Str(StrHandle(child))
            }
            NodeKind::NonTerminal(NonTerminalKind::Integer) => {
                KeyBaseView::Integer(IntegerHandle(child))
            }
            NodeKind::Terminal(kind) => {
                return Err(ViewConstructionError::UnexpectedTerminal {
                    node: child,
                    terminal: kind,
                });
            }
            NodeKind::NonTerminal(kind) => {
                return Err(ViewConstructionError::UnexpectedNonTerminal {
                    node: child,
                    non_terminal: kind,
                });
            }
        };
        if let Some(child) = children.next() {
            return Err(ViewConstructionError::UnexpectedExtraNode {
                node: child,
            });
        }
        Ok(variant)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyBaseView {
    Ident(IdentHandle),
    ExtensionNameSpace(ExtensionNameSpaceHandle),
    Str(StrHandle),
    Integer(IntegerHandle),
}
impl KeyBaseView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeyOptHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for KeyOptHandle {
    type View = Option<ArrayMarkerHandle>;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::KeyOpt))?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::KeyOpt
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        if tree.has_no_children(self.0) {
            return Ok(None);
        }
        let [child] = tree
            .collect_nodes(
                self.0,
                [NodeKind::NonTerminal(NonTerminalKind::ArrayMarker)],
                visit_ignored,
            )?;
        Ok(
            Some(
                ArrayMarkerHandle::new(
                        child,
                        NodeKind::NonTerminal(NonTerminalKind::ArrayMarker),
                    )
                    .map_err(|e| e.into_any_error::<E>())?,
            ),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeysHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for KeysHandle {
    type View = KeysView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Keys))?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Keys
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [key, keys_list] = tree
            .collect_nodes(
                self.0,
                [
                    NodeKind::NonTerminal(NonTerminalKind::Key),
                    NodeKind::NonTerminal(NonTerminalKind::KeysList),
                ],
                visit_ignored,
            )?;
        Ok(KeysView {
            key: KeyHandle(key),
            keys_list: KeysListHandle(keys_list),
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeysView {
    pub key: KeyHandle,
    pub keys_list: KeysListHandle,
}
impl KeysView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeysListHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for KeysListHandle {
    type View = Option<KeysListView>;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::KeysList))?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::KeysList
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        if tree.has_no_children(self.0) {
            return Ok(None);
        }
        let [dot, key, keys_list] = tree
            .collect_nodes(
                self.0,
                [
                    NodeKind::NonTerminal(NonTerminalKind::Dot),
                    NodeKind::NonTerminal(NonTerminalKind::Key),
                    NodeKind::NonTerminal(NonTerminalKind::KeysList),
                ],
                visit_ignored,
            )?;
        Ok(
            Some(KeysListView {
                dot: DotHandle(dot),
                key: KeyHandle(key),
                keys_list: KeysListHandle(keys_list),
            }),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeysListView {
    pub dot: DotHandle,
    pub key: KeyHandle,
    pub keys_list: KeysListHandle,
}
impl KeysListView {
    pub fn get_all(&self, tree: &Cst) -> Result<Vec<KeysListItem>, CstConstructError> {
        self.get_all_with_visit(tree, |_, _, _| Ok(()))
    }
    pub fn get_all_with_visit<E>(
        &self,
        tree: &Cst,
        mut visit_ignored: impl FnMut(
            CstNodeId,
            TerminalKind,
            TerminalData,
        ) -> Result<(), E>,
    ) -> Result<Vec<KeysListItem>, CstConstructError<E>> {
        let mut items = Vec::new();
        let mut current_view = Some(*self);
        while let Some(item) = current_view {
            let Self { dot, key, .. } = item;
            items.push(KeysListItem { dot, key });
            current_view = item.keys_list.get_view_with_visit(tree, &mut visit_ignored)?;
        }
        Ok(items)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeysListItem {
    pub dot: DotHandle,
    pub key: KeyHandle,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NamedCodeHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for NamedCodeHandle {
    type View = NamedCodeView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(
            index,
            kind,
            NodeKind::NonTerminal(NonTerminalKind::NamedCode),
        )?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::NamedCode
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [named_code] = tree
            .collect_nodes(
                self.0,
                [NodeKind::Terminal(TerminalKind::NamedCode)],
                visit_ignored,
            )?;
        Ok(NamedCodeView {
            named_code: NamedCode(named_code),
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NamedCodeView {
    pub named_code: NamedCode,
}
impl NamedCodeView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NamedCodeBlockHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for NamedCodeBlockHandle {
    type View = NamedCodeBlockView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(
            index,
            kind,
            NodeKind::NonTerminal(NonTerminalKind::NamedCodeBlock),
        )?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::NamedCodeBlock
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [named_code_block_begin, code_block_tail_common] = tree
            .collect_nodes(
                self.0,
                [
                    NodeKind::NonTerminal(NonTerminalKind::NamedCodeBlockBegin),
                    NodeKind::NonTerminal(NonTerminalKind::CodeBlockTailCommon),
                ],
                visit_ignored,
            )?;
        Ok(NamedCodeBlockView {
            named_code_block_begin: NamedCodeBlockBeginHandle(named_code_block_begin),
            code_block_tail_common: CodeBlockTailCommonHandle(code_block_tail_common),
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NamedCodeBlockView {
    pub named_code_block_begin: NamedCodeBlockBeginHandle,
    pub code_block_tail_common: CodeBlockTailCommonHandle,
}
impl NamedCodeBlockView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NamedCodeBlockBeginHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for NamedCodeBlockBeginHandle {
    type View = NamedCodeBlockBeginView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(
            index,
            kind,
            NodeKind::NonTerminal(NonTerminalKind::NamedCodeBlockBegin),
        )?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::NamedCodeBlockBegin
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [named_code_block_begin] = tree
            .collect_nodes(
                self.0,
                [NodeKind::Terminal(TerminalKind::NamedCodeBlockBegin)],
                visit_ignored,
            )?;
        Ok(NamedCodeBlockBeginView {
            named_code_block_begin: NamedCodeBlockBegin(named_code_block_begin),
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NamedCodeBlockBeginView {
    pub named_code_block_begin: NamedCodeBlockBegin,
}
impl NamedCodeBlockBeginView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NewlineHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for NewlineHandle {
    type View = NewlineView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Newline))?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Newline
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [newline] = tree
            .collect_nodes(
                self.0,
                [NodeKind::Terminal(TerminalKind::Newline)],
                visit_ignored,
            )?;
        Ok(NewlineView {
            newline: Newline(newline),
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NewlineView {
    pub newline: Newline,
}
impl NewlineView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NullHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for NullHandle {
    type View = NullView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Null))?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Null
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [null] = tree
            .collect_nodes(
                self.0,
                [NodeKind::Terminal(TerminalKind::Null)],
                visit_ignored,
            )?;
        Ok(NullView { null: Null(null) })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NullView {
    pub null: Null,
}
impl NullView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for ObjectHandle {
    type View = ObjectView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Object))?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Object
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [begin, object_list, end] = tree
            .collect_nodes(
                self.0,
                [
                    NodeKind::NonTerminal(NonTerminalKind::Begin),
                    NodeKind::NonTerminal(NonTerminalKind::ObjectList),
                    NodeKind::NonTerminal(NonTerminalKind::End),
                ],
                visit_ignored,
            )?;
        Ok(ObjectView {
            begin: BeginHandle(begin),
            object_list: ObjectListHandle(object_list),
            end: EndHandle(end),
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectView {
    pub begin: BeginHandle,
    pub object_list: ObjectListHandle,
    pub end: EndHandle,
}
impl ObjectView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectListHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for ObjectListHandle {
    type View = Option<ObjectListView>;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(
            index,
            kind,
            NodeKind::NonTerminal(NonTerminalKind::ObjectList),
        )?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::ObjectList
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        if tree.has_no_children(self.0) {
            return Ok(None);
        }
        let [key, bind, value, object_opt, object_list] = tree
            .collect_nodes(
                self.0,
                [
                    NodeKind::NonTerminal(NonTerminalKind::Key),
                    NodeKind::NonTerminal(NonTerminalKind::Bind),
                    NodeKind::NonTerminal(NonTerminalKind::Value),
                    NodeKind::NonTerminal(NonTerminalKind::ObjectOpt),
                    NodeKind::NonTerminal(NonTerminalKind::ObjectList),
                ],
                visit_ignored,
            )?;
        Ok(
            Some(ObjectListView {
                key: KeyHandle(key),
                bind: BindHandle(bind),
                value: ValueHandle(value),
                object_opt: ObjectOptHandle(object_opt),
                object_list: ObjectListHandle(object_list),
            }),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectListView {
    pub key: KeyHandle,
    pub bind: BindHandle,
    pub value: ValueHandle,
    pub object_opt: ObjectOptHandle,
    pub object_list: ObjectListHandle,
}
impl ObjectListView {
    pub fn get_all(&self, tree: &Cst) -> Result<Vec<ObjectListItem>, CstConstructError> {
        self.get_all_with_visit(tree, |_, _, _| Ok(()))
    }
    pub fn get_all_with_visit<E>(
        &self,
        tree: &Cst,
        mut visit_ignored: impl FnMut(
            CstNodeId,
            TerminalKind,
            TerminalData,
        ) -> Result<(), E>,
    ) -> Result<Vec<ObjectListItem>, CstConstructError<E>> {
        let mut items = Vec::new();
        let mut current_view = Some(*self);
        while let Some(item) = current_view {
            let Self { key, bind, value, object_opt, .. } = item;
            items
                .push(ObjectListItem {
                    key,
                    bind,
                    value,
                    object_opt,
                });
            current_view = item
                .object_list
                .get_view_with_visit(tree, &mut visit_ignored)?;
        }
        Ok(items)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectListItem {
    pub key: KeyHandle,
    pub bind: BindHandle,
    pub value: ValueHandle,
    pub object_opt: ObjectOptHandle,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectOptHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for ObjectOptHandle {
    type View = Option<CommaHandle>;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(
            index,
            kind,
            NodeKind::NonTerminal(NonTerminalKind::ObjectOpt),
        )?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::ObjectOpt
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        if tree.has_no_children(self.0) {
            return Ok(None);
        }
        let [child] = tree
            .collect_nodes(
                self.0,
                [NodeKind::NonTerminal(NonTerminalKind::Comma)],
                visit_ignored,
            )?;
        Ok(
            Some(
                CommaHandle::new(child, NodeKind::NonTerminal(NonTerminalKind::Comma))
                    .map_err(|e| e.into_any_error::<E>())?,
            ),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QuoteHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for QuoteHandle {
    type View = QuoteView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Quote))?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Quote
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [quote] = tree
            .collect_nodes(
                self.0,
                [NodeKind::Terminal(TerminalKind::Quote)],
                visit_ignored,
            )?;
        Ok(QuoteView { quote: Quote(quote) })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuoteView {
    pub quote: Quote,
}
impl QuoteView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SectionHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for SectionHandle {
    type View = SectionView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Section))?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Section
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [at, keys, section_list] = tree
            .collect_nodes(
                self.0,
                [
                    NodeKind::NonTerminal(NonTerminalKind::At),
                    NodeKind::NonTerminal(NonTerminalKind::Keys),
                    NodeKind::NonTerminal(NonTerminalKind::SectionList),
                ],
                visit_ignored,
            )?;
        Ok(SectionView {
            at: AtHandle(at),
            keys: KeysHandle(keys),
            section_list: SectionListHandle(section_list),
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SectionView {
    pub at: AtHandle,
    pub keys: KeysHandle,
    pub section_list: SectionListHandle,
}
impl SectionView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SectionBindingHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for SectionBindingHandle {
    type View = SectionBindingView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(
            index,
            kind,
            NodeKind::NonTerminal(NonTerminalKind::SectionBinding),
        )?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::SectionBinding
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [begin, swon, end] = tree
            .collect_nodes(
                self.0,
                [
                    NodeKind::NonTerminal(NonTerminalKind::Begin),
                    NodeKind::NonTerminal(NonTerminalKind::Swon),
                    NodeKind::NonTerminal(NonTerminalKind::End),
                ],
                visit_ignored,
            )?;
        Ok(SectionBindingView {
            begin: BeginHandle(begin),
            swon: SwonHandle(swon),
            end: EndHandle(end),
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SectionBindingView {
    pub begin: BeginHandle,
    pub swon: SwonHandle,
    pub end: EndHandle,
}
impl SectionBindingView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SectionListHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for SectionListHandle {
    type View = Option<SectionListView>;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(
            index,
            kind,
            NodeKind::NonTerminal(NonTerminalKind::SectionList),
        )?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::SectionList
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        if tree.has_no_children(self.0) {
            return Ok(None);
        }
        let [binding, section_list] = tree
            .collect_nodes(
                self.0,
                [
                    NodeKind::NonTerminal(NonTerminalKind::Binding),
                    NodeKind::NonTerminal(NonTerminalKind::SectionList),
                ],
                visit_ignored,
            )?;
        Ok(
            Some(SectionListView {
                binding: BindingHandle(binding),
                section_list: SectionListHandle(section_list),
            }),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SectionListView {
    pub binding: BindingHandle,
    pub section_list: SectionListHandle,
}
impl SectionListView {
    pub fn get_all(&self, tree: &Cst) -> Result<Vec<BindingHandle>, CstConstructError> {
        self.get_all_with_visit(tree, |_, _, _| Ok(()))
    }
    pub fn get_all_with_visit<E>(
        &self,
        tree: &Cst,
        mut visit_ignored: impl FnMut(
            CstNodeId,
            TerminalKind,
            TerminalData,
        ) -> Result<(), E>,
    ) -> Result<Vec<BindingHandle>, CstConstructError<E>> {
        let mut items = Vec::new();
        let mut current_view = Some(*self);
        while let Some(item) = current_view {
            let Self { binding, .. } = item;
            items.push(binding);
            current_view = item
                .section_list
                .get_view_with_visit(tree, &mut visit_ignored)?;
        }
        Ok(items)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StrHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for StrHandle {
    type View = StrView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Str))?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Str
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [quote, in_str, quote2] = tree
            .collect_nodes(
                self.0,
                [
                    NodeKind::NonTerminal(NonTerminalKind::Quote),
                    NodeKind::NonTerminal(NonTerminalKind::InStr),
                    NodeKind::NonTerminal(NonTerminalKind::Quote),
                ],
                visit_ignored,
            )?;
        Ok(StrView {
            quote: QuoteHandle(quote),
            in_str: InStrHandle(in_str),
            quote2: QuoteHandle(quote2),
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StrView {
    pub quote: QuoteHandle,
    pub in_str: InStrHandle,
    pub quote2: QuoteHandle,
}
impl StrView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StrContinuesHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for StrContinuesHandle {
    type View = StrContinuesView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(
            index,
            kind,
            NodeKind::NonTerminal(NonTerminalKind::StrContinues),
        )?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::StrContinues
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [str, str_continues_list] = tree
            .collect_nodes(
                self.0,
                [
                    NodeKind::NonTerminal(NonTerminalKind::Str),
                    NodeKind::NonTerminal(NonTerminalKind::StrContinuesList),
                ],
                visit_ignored,
            )?;
        Ok(StrContinuesView {
            str: StrHandle(str),
            str_continues_list: StrContinuesListHandle(str_continues_list),
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StrContinuesView {
    pub str: StrHandle,
    pub str_continues_list: StrContinuesListHandle,
}
impl StrContinuesView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StrContinuesListHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for StrContinuesListHandle {
    type View = Option<StrContinuesListView>;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(
            index,
            kind,
            NodeKind::NonTerminal(NonTerminalKind::StrContinuesList),
        )?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::StrContinuesList
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        if tree.has_no_children(self.0) {
            return Ok(None);
        }
        let [r#continue, str, str_continues_list] = tree
            .collect_nodes(
                self.0,
                [
                    NodeKind::NonTerminal(NonTerminalKind::Continue),
                    NodeKind::NonTerminal(NonTerminalKind::Str),
                    NodeKind::NonTerminal(NonTerminalKind::StrContinuesList),
                ],
                visit_ignored,
            )?;
        Ok(
            Some(StrContinuesListView {
                r#continue: ContinueHandle(r#continue),
                str: StrHandle(str),
                str_continues_list: StrContinuesListHandle(str_continues_list),
            }),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StrContinuesListView {
    pub r#continue: ContinueHandle,
    pub str: StrHandle,
    pub str_continues_list: StrContinuesListHandle,
}
impl StrContinuesListView {
    pub fn get_all(
        &self,
        tree: &Cst,
    ) -> Result<Vec<StrContinuesListItem>, CstConstructError> {
        self.get_all_with_visit(tree, |_, _, _| Ok(()))
    }
    pub fn get_all_with_visit<E>(
        &self,
        tree: &Cst,
        mut visit_ignored: impl FnMut(
            CstNodeId,
            TerminalKind,
            TerminalData,
        ) -> Result<(), E>,
    ) -> Result<Vec<StrContinuesListItem>, CstConstructError<E>> {
        let mut items = Vec::new();
        let mut current_view = Some(*self);
        while let Some(item) = current_view {
            let Self { r#continue, str, .. } = item;
            items
                .push(StrContinuesListItem {
                    r#continue,
                    str,
                });
            current_view = item
                .str_continues_list
                .get_view_with_visit(tree, &mut visit_ignored)?;
        }
        Ok(items)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StrContinuesListItem {
    pub r#continue: ContinueHandle,
    pub str: StrHandle,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SwonHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for SwonHandle {
    type View = SwonView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Swon))?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Swon
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [swon_bindings, swon_sections] = tree
            .collect_nodes(
                self.0,
                [
                    NodeKind::NonTerminal(NonTerminalKind::SwonList),
                    NodeKind::NonTerminal(NonTerminalKind::SwonList0),
                ],
                visit_ignored,
            )?;
        Ok(SwonView {
            swon_bindings: SwonBindingsHandle(swon_bindings),
            swon_sections: SwonSectionsHandle(swon_sections),
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SwonView {
    pub swon_bindings: SwonBindingsHandle,
    pub swon_sections: SwonSectionsHandle,
}
impl SwonView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SwonBindingsHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for SwonBindingsHandle {
    type View = Option<SwonBindingsView>;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::SwonList))?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::SwonList
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        if tree.has_no_children(self.0) {
            return Ok(None);
        }
        let [binding, swon_bindings] = tree
            .collect_nodes(
                self.0,
                [
                    NodeKind::NonTerminal(NonTerminalKind::Binding),
                    NodeKind::NonTerminal(NonTerminalKind::SwonList),
                ],
                visit_ignored,
            )?;
        Ok(
            Some(SwonBindingsView {
                binding: BindingHandle(binding),
                swon_bindings: SwonBindingsHandle(swon_bindings),
            }),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SwonBindingsView {
    pub binding: BindingHandle,
    pub swon_bindings: SwonBindingsHandle,
}
impl SwonBindingsView {
    pub fn get_all(&self, tree: &Cst) -> Result<Vec<BindingHandle>, CstConstructError> {
        self.get_all_with_visit(tree, |_, _, _| Ok(()))
    }
    pub fn get_all_with_visit<E>(
        &self,
        tree: &Cst,
        mut visit_ignored: impl FnMut(
            CstNodeId,
            TerminalKind,
            TerminalData,
        ) -> Result<(), E>,
    ) -> Result<Vec<BindingHandle>, CstConstructError<E>> {
        let mut items = Vec::new();
        let mut current_view = Some(*self);
        while let Some(item) = current_view {
            let Self { binding, .. } = item;
            items.push(binding);
            current_view = item
                .swon_bindings
                .get_view_with_visit(tree, &mut visit_ignored)?;
        }
        Ok(items)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SwonSectionsHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for SwonSectionsHandle {
    type View = Option<SwonSectionsView>;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(
            index,
            kind,
            NodeKind::NonTerminal(NonTerminalKind::SwonList0),
        )?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::SwonList0
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        if tree.has_no_children(self.0) {
            return Ok(None);
        }
        let [section, swon_sections] = tree
            .collect_nodes(
                self.0,
                [
                    NodeKind::NonTerminal(NonTerminalKind::Section),
                    NodeKind::NonTerminal(NonTerminalKind::SwonList0),
                ],
                visit_ignored,
            )?;
        Ok(
            Some(SwonSectionsView {
                section: SectionHandle(section),
                swon_sections: SwonSectionsHandle(swon_sections),
            }),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SwonSectionsView {
    pub section: SectionHandle,
    pub swon_sections: SwonSectionsHandle,
}
impl SwonSectionsView {
    pub fn get_all(&self, tree: &Cst) -> Result<Vec<SectionHandle>, CstConstructError> {
        self.get_all_with_visit(tree, |_, _, _| Ok(()))
    }
    pub fn get_all_with_visit<E>(
        &self,
        tree: &Cst,
        mut visit_ignored: impl FnMut(
            CstNodeId,
            TerminalKind,
            TerminalData,
        ) -> Result<(), E>,
    ) -> Result<Vec<SectionHandle>, CstConstructError<E>> {
        let mut items = Vec::new();
        let mut current_view = Some(*self);
        while let Some(item) = current_view {
            let Self { section, .. } = item;
            items.push(section);
            current_view = item
                .swon_sections
                .get_view_with_visit(tree, &mut visit_ignored)?;
        }
        Ok(items)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for TextHandle {
    type View = TextView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Text))?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Text
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [text] = tree
            .collect_nodes(
                self.0,
                [NodeKind::Terminal(TerminalKind::Text)],
                visit_ignored,
            )?;
        Ok(TextView { text: Text(text) })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextView {
    pub text: Text,
}
impl TextView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextBindingHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for TextBindingHandle {
    type View = TextBindingView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(
            index,
            kind,
            NodeKind::NonTerminal(NonTerminalKind::TextBinding),
        )?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::TextBinding
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [text_start, text_binding_opt, text, newline] = tree
            .collect_nodes(
                self.0,
                [
                    NodeKind::NonTerminal(NonTerminalKind::TextStart),
                    NodeKind::NonTerminal(NonTerminalKind::TextBindingOpt),
                    NodeKind::NonTerminal(NonTerminalKind::Text),
                    NodeKind::NonTerminal(NonTerminalKind::Newline),
                ],
                visit_ignored,
            )?;
        Ok(TextBindingView {
            text_start: TextStartHandle(text_start),
            text_binding_opt: TextBindingOptHandle(text_binding_opt),
            text: TextHandle(text),
            newline: NewlineHandle(newline),
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextBindingView {
    pub text_start: TextStartHandle,
    pub text_binding_opt: TextBindingOptHandle,
    pub text: TextHandle,
    pub newline: NewlineHandle,
}
impl TextBindingView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextBindingOptHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for TextBindingOptHandle {
    type View = Option<WsHandle>;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(
            index,
            kind,
            NodeKind::NonTerminal(NonTerminalKind::TextBindingOpt),
        )?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::TextBindingOpt
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        if tree.has_no_children(self.0) {
            return Ok(None);
        }
        let [child] = tree
            .collect_nodes(
                self.0,
                [NodeKind::NonTerminal(NonTerminalKind::Ws)],
                visit_ignored,
            )?;
        Ok(
            Some(
                WsHandle::new(child, NodeKind::NonTerminal(NonTerminalKind::Ws))
                    .map_err(|e| e.into_any_error::<E>())?,
            ),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextStartHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for TextStartHandle {
    type View = TextStartView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(
            index,
            kind,
            NodeKind::NonTerminal(NonTerminalKind::TextStart),
        )?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::TextStart
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [text_start] = tree
            .collect_nodes(
                self.0,
                [NodeKind::Terminal(TerminalKind::TextStart)],
                visit_ignored,
            )?;
        Ok(TextStartView {
            text_start: TextStart(text_start),
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextStartView {
    pub text_start: TextStart,
}
impl TextStartView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TrueHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for TrueHandle {
    type View = TrueView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::True))?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::True
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [r#true] = tree
            .collect_nodes(
                self.0,
                [NodeKind::Terminal(TerminalKind::True)],
                visit_ignored,
            )?;
        Ok(TrueView { r#true: True(r#true) })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrueView {
    pub r#true: True,
}
impl TrueView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypedQuoteHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for TypedQuoteHandle {
    type View = TypedQuoteView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(
            index,
            kind,
            NodeKind::NonTerminal(NonTerminalKind::TypedQuote),
        )?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::TypedQuote
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [typed_quote] = tree
            .collect_nodes(
                self.0,
                [NodeKind::Terminal(TerminalKind::TypedQuote)],
                visit_ignored,
            )?;
        Ok(TypedQuoteView {
            typed_quote: TypedQuote(typed_quote),
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypedQuoteView {
    pub typed_quote: TypedQuote,
}
impl TypedQuoteView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypedStrHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for TypedStrHandle {
    type View = TypedStrView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::TypedStr))?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::TypedStr
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [typed_quote, in_str, quote] = tree
            .collect_nodes(
                self.0,
                [
                    NodeKind::NonTerminal(NonTerminalKind::TypedQuote),
                    NodeKind::NonTerminal(NonTerminalKind::InStr),
                    NodeKind::NonTerminal(NonTerminalKind::Quote),
                ],
                visit_ignored,
            )?;
        Ok(TypedStrView {
            typed_quote: TypedQuoteHandle(typed_quote),
            in_str: InStrHandle(in_str),
            quote: QuoteHandle(quote),
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypedStrView {
    pub typed_quote: TypedQuoteHandle,
    pub in_str: InStrHandle,
    pub quote: QuoteHandle,
}
impl TypedStrView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ValueHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for ValueHandle {
    type View = ValueView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Value))?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Value
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let mut children = tree.children(self.0);
        let Some(child) = children.next() else {
            return Err(ViewConstructionError::UnexpectedEndOfChildren {
                parent: self.0,
            });
        };
        let Some(child_data) = tree.node_data(child) else {
            return Err(ViewConstructionError::NodeIdNotFound {
                node: child,
            });
        };
        let variant = match child_data.node_kind() {
            NodeKind::NonTerminal(NonTerminalKind::Object) => {
                ValueView::Object(ObjectHandle(child))
            }
            NodeKind::NonTerminal(NonTerminalKind::Array) => {
                ValueView::Array(ArrayHandle(child))
            }
            NodeKind::NonTerminal(NonTerminalKind::Integer) => {
                ValueView::Integer(IntegerHandle(child))
            }
            NodeKind::NonTerminal(NonTerminalKind::Boolean) => {
                ValueView::Boolean(BooleanHandle(child))
            }
            NodeKind::NonTerminal(NonTerminalKind::Null) => {
                ValueView::Null(NullHandle(child))
            }
            NodeKind::NonTerminal(NonTerminalKind::StrContinues) => {
                ValueView::StrContinues(StrContinuesHandle(child))
            }
            NodeKind::NonTerminal(NonTerminalKind::TypedStr) => {
                ValueView::TypedStr(TypedStrHandle(child))
            }
            NodeKind::NonTerminal(NonTerminalKind::Hole) => {
                ValueView::Hole(HoleHandle(child))
            }
            NodeKind::NonTerminal(NonTerminalKind::NamedCodeBlock) => {
                ValueView::NamedCodeBlock(NamedCodeBlockHandle(child))
            }
            NodeKind::NonTerminal(NonTerminalKind::CodeBlock) => {
                ValueView::CodeBlock(CodeBlockHandle(child))
            }
            NodeKind::NonTerminal(NonTerminalKind::NamedCode) => {
                ValueView::NamedCode(NamedCodeHandle(child))
            }
            NodeKind::NonTerminal(NonTerminalKind::Code) => {
                ValueView::Code(CodeHandle(child))
            }
            NodeKind::Terminal(kind) => {
                return Err(ViewConstructionError::UnexpectedTerminal {
                    node: child,
                    terminal: kind,
                });
            }
            NodeKind::NonTerminal(kind) => {
                return Err(ViewConstructionError::UnexpectedNonTerminal {
                    node: child,
                    non_terminal: kind,
                });
            }
        };
        if let Some(child) = children.next() {
            return Err(ViewConstructionError::UnexpectedExtraNode {
                node: child,
            });
        }
        Ok(variant)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueView {
    Object(ObjectHandle),
    Array(ArrayHandle),
    Integer(IntegerHandle),
    Boolean(BooleanHandle),
    Null(NullHandle),
    StrContinues(StrContinuesHandle),
    TypedStr(TypedStrHandle),
    Hole(HoleHandle),
    NamedCodeBlock(NamedCodeBlockHandle),
    CodeBlock(CodeBlockHandle),
    NamedCode(NamedCodeHandle),
    Code(CodeHandle),
}
impl ValueView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ValueBindingHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for ValueBindingHandle {
    type View = ValueBindingView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(
            index,
            kind,
            NodeKind::NonTerminal(NonTerminalKind::ValueBinding),
        )?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::ValueBinding
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [bind, value] = tree
            .collect_nodes(
                self.0,
                [
                    NodeKind::NonTerminal(NonTerminalKind::Bind),
                    NodeKind::NonTerminal(NonTerminalKind::Value),
                ],
                visit_ignored,
            )?;
        Ok(ValueBindingView {
            bind: BindHandle(bind),
            value: ValueHandle(value),
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValueBindingView {
    pub bind: BindHandle,
    pub value: ValueHandle,
}
impl ValueBindingView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WsHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for WsHandle {
    type View = WsView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Ws))?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Ws
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [ws] = tree
            .collect_nodes(
                self.0,
                [NodeKind::Terminal(TerminalKind::Ws)],
                visit_ignored,
            )?;
        Ok(WsView { ws: Ws(ws) })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WsView {
    pub ws: Ws,
}
impl WsView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RootHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for RootHandle {
    type View = RootView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Root))?;
        Ok(Self(index))
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Root
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        Ok(self.get_view_with_visit(tree, |_, _, _| Ok(()))?)
    }
    fn get_view_with_visit<E>(
        &self,
        tree: &Cst,
        visit_ignored: impl FnMut(CstNodeId, TerminalKind, TerminalData) -> Result<(), E>,
    ) -> Result<Self::View, CstConstructError<E>> {
        let [swon] = tree
            .collect_nodes(
                self.0,
                [NodeKind::NonTerminal(NonTerminalKind::Swon)],
                visit_ignored,
            )?;
        Ok(RootView { swon: SwonHandle(swon) })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RootView {
    pub swon: SwonHandle,
}
impl RootView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NewLine(pub CstNodeId);
impl NewLine {
    pub fn kind(&self) -> TerminalKind {
        TerminalKind::NewLine
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Whitespace(pub CstNodeId);
impl Whitespace {
    pub fn kind(&self) -> TerminalKind {
        TerminalKind::Whitespace
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LineComment(pub CstNodeId);
impl LineComment {
    pub fn kind(&self) -> TerminalKind {
        TerminalKind::LineComment
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockComment(pub CstNodeId);
impl BlockComment {
    pub fn kind(&self) -> TerminalKind {
        TerminalKind::BlockComment
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Integer(pub CstNodeId);
impl Integer {
    pub fn kind(&self) -> TerminalKind {
        TerminalKind::Integer
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct True(pub CstNodeId);
impl True {
    pub fn kind(&self) -> TerminalKind {
        TerminalKind::True
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct False(pub CstNodeId);
impl False {
    pub fn kind(&self) -> TerminalKind {
        TerminalKind::False
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Null(pub CstNodeId);
impl Null {
    pub fn kind(&self) -> TerminalKind {
        TerminalKind::Null
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Hole(pub CstNodeId);
impl Hole {
    pub fn kind(&self) -> TerminalKind {
        TerminalKind::Hole
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Quote(pub CstNodeId);
impl Quote {
    pub fn kind(&self) -> TerminalKind {
        TerminalKind::Quote
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypedQuote(pub CstNodeId);
impl TypedQuote {
    pub fn kind(&self) -> TerminalKind {
        TerminalKind::TypedQuote
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InStr(pub CstNodeId);
impl InStr {
    pub fn kind(&self) -> TerminalKind {
        TerminalKind::InStr
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Text(pub CstNodeId);
impl Text {
    pub fn kind(&self) -> TerminalKind {
        TerminalKind::Text
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NamedCode(pub CstNodeId);
impl NamedCode {
    pub fn kind(&self) -> TerminalKind {
        TerminalKind::NamedCode
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Code(pub CstNodeId);
impl Code {
    pub fn kind(&self) -> TerminalKind {
        TerminalKind::Code
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Newline(pub CstNodeId);
impl Newline {
    pub fn kind(&self) -> TerminalKind {
        TerminalKind::Newline
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ws(pub CstNodeId);
impl Ws {
    pub fn kind(&self) -> TerminalKind {
        TerminalKind::Ws
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct At(pub CstNodeId);
impl At {
    pub fn kind(&self) -> TerminalKind {
        TerminalKind::At
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Dollar(pub CstNodeId);
impl Dollar {
    pub fn kind(&self) -> TerminalKind {
        TerminalKind::Dollar
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Dot(pub CstNodeId);
impl Dot {
    pub fn kind(&self) -> TerminalKind {
        TerminalKind::Dot
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LBrace(pub CstNodeId);
impl LBrace {
    pub fn kind(&self) -> TerminalKind {
        TerminalKind::LBrace
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RBrace(pub CstNodeId);
impl RBrace {
    pub fn kind(&self) -> TerminalKind {
        TerminalKind::RBrace
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LBracket(pub CstNodeId);
impl LBracket {
    pub fn kind(&self) -> TerminalKind {
        TerminalKind::LBracket
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RBracket(pub CstNodeId);
impl RBracket {
    pub fn kind(&self) -> TerminalKind {
        TerminalKind::RBracket
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Bind(pub CstNodeId);
impl Bind {
    pub fn kind(&self) -> TerminalKind {
        TerminalKind::Bind
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Comma(pub CstNodeId);
impl Comma {
    pub fn kind(&self) -> TerminalKind {
        TerminalKind::Comma
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Esc(pub CstNodeId);
impl Esc {
    pub fn kind(&self) -> TerminalKind {
        TerminalKind::Esc
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextStart(pub CstNodeId);
impl TextStart {
    pub fn kind(&self) -> TerminalKind {
        TerminalKind::TextStart
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ident(pub CstNodeId);
impl Ident {
    pub fn kind(&self) -> TerminalKind {
        TerminalKind::Ident
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NamedCodeBlockBegin(pub CstNodeId);
impl NamedCodeBlockBegin {
    pub fn kind(&self) -> TerminalKind {
        TerminalKind::NamedCodeBlockBegin
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CodeBlockDelimiter(pub CstNodeId);
impl CodeBlockDelimiter {
    pub fn kind(&self) -> TerminalKind {
        TerminalKind::CodeBlockDelimiter
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CodeBlockLine(pub CstNodeId);
impl CodeBlockLine {
    pub fn kind(&self) -> TerminalKind {
        TerminalKind::CodeBlockLine
    }
}
