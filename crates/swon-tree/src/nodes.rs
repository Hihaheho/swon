#![allow(unused_variables)]
use super::tree::{
    TerminalHandle, NonTerminalHandle, RecursiveView, CstNodeId, ViewConstructionError,
    CstFacade,
};
use super::visitor::BuiltinTerminalVisitor;
use crate::CstConstructError;
use super::node_kind::{TerminalKind, NonTerminalKind, NodeKind};
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArrayHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle for ArrayHandle {
    type View = ArrayView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::Array)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Array
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::ArrayBegin),
                NodeKind::NonTerminal(NonTerminalKind::ArrayList),
                NodeKind::NonTerminal(NonTerminalKind::ArrayEnd),
            ],
            |[array_begin, array_list, array_end], visit_ignored| Ok(
                visit(
                    ArrayView {
                        array_begin: ArrayBeginHandle(array_begin),
                        array_list: ArrayListHandle(array_list),
                        array_end: ArrayEndHandle(array_end),
                    },
                    visit_ignored,
                ),
            ),
            visit_ignored,
        )
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
impl NonTerminalHandle for ArrayBeginHandle {
    type View = ArrayBeginView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::ArrayBegin)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::ArrayBegin
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        tree.collect_nodes(
            self.0,
            [NodeKind::Terminal(TerminalKind::LBracket)],
            |[l_bracket], visit_ignored| Ok(
                visit(
                    ArrayBeginView {
                        l_bracket: LBracket(l_bracket),
                    },
                    visit_ignored,
                ),
            ),
            visit_ignored,
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArrayBeginView {
    pub l_bracket: LBracket,
}
impl ArrayBeginView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArrayEndHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle for ArrayEndHandle {
    type View = ArrayEndView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::ArrayEnd)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::ArrayEnd
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        tree.collect_nodes(
            self.0,
            [NodeKind::Terminal(TerminalKind::RBracket)],
            |[r_bracket], visit_ignored| Ok(
                visit(
                    ArrayEndView {
                        r_bracket: RBracket(r_bracket),
                    },
                    visit_ignored,
                ),
            ),
            visit_ignored,
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArrayEndView {
    pub r_bracket: RBracket,
}
impl ArrayEndView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArrayListHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle for ArrayListHandle {
    type View = Option<ArrayListView>;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::ArrayList)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::ArrayList
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        if tree.has_no_children(self.0) {
            return Ok(visit(None, visit_ignored).0);
        }
        tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::Value),
                NodeKind::NonTerminal(NonTerminalKind::ArrayOpt),
                NodeKind::NonTerminal(NonTerminalKind::ArrayList),
            ],
            |[value, array_opt, array_list], visit_ignored| Ok(
                visit(
                    Some(ArrayListView {
                        value: ValueHandle(value),
                        array_opt: ArrayOptHandle(array_opt),
                        array_list: ArrayListHandle(array_list),
                    }),
                    visit_ignored,
                ),
            ),
            visit_ignored,
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArrayListView {
    pub value: ValueHandle,
    pub array_opt: ArrayOptHandle,
    pub array_list: ArrayListHandle,
}
impl<F: CstFacade> RecursiveView<F> for ArrayListView {
    type Item = ArrayListItem;
    fn get_all_with_visit<E>(
        &self,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Vec<Self::Item>, CstConstructError<E>> {
        let mut items = Vec::new();
        let mut current_view = Some(*self);
        while let Some(item) = current_view {
            let Self { value, array_opt, .. } = item;
            items.push(ArrayListItem { value, array_opt });
            item.array_list
                .get_view_with_visit(
                    tree,
                    |view, visit_ignored| {
                        current_view = view;
                        ((), visit_ignored)
                    },
                    visit_ignored,
                )?;
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
impl NonTerminalHandle for ArrayMarkerHandle {
    type View = ArrayMarkerView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::ArrayMarker)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::ArrayMarker
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::ArrayBegin),
                NodeKind::NonTerminal(NonTerminalKind::ArrayMarkerOpt),
                NodeKind::NonTerminal(NonTerminalKind::ArrayEnd),
            ],
            |[array_begin, array_marker_opt, array_end], visit_ignored| Ok(
                visit(
                    ArrayMarkerView {
                        array_begin: ArrayBeginHandle(array_begin),
                        array_marker_opt: ArrayMarkerOptHandle(array_marker_opt),
                        array_end: ArrayEndHandle(array_end),
                    },
                    visit_ignored,
                ),
            ),
            visit_ignored,
        )
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
impl NonTerminalHandle for ArrayMarkerOptHandle {
    type View = Option<IntegerHandle>;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::ArrayMarkerOpt)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::ArrayMarkerOpt
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        if tree.has_no_children(self.0) {
            return Ok(visit(None, visit_ignored).0);
        }
        tree.collect_nodes(
            self.0,
            [NodeKind::NonTerminal(NonTerminalKind::Integer)],
            |[child], visit_ignored| Ok(
                visit(
                    Some(IntegerHandle::new_with_visit(child, tree, visit_ignored)?),
                    visit_ignored,
                ),
            ),
            visit_ignored,
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArrayOptHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle for ArrayOptHandle {
    type View = Option<CommaHandle>;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::ArrayOpt)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::ArrayOpt
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        if tree.has_no_children(self.0) {
            return Ok(visit(None, visit_ignored).0);
        }
        tree.collect_nodes(
            self.0,
            [NodeKind::NonTerminal(NonTerminalKind::Comma)],
            |[child], visit_ignored| Ok(
                visit(
                    Some(CommaHandle::new_with_visit(child, tree, visit_ignored)?),
                    visit_ignored,
                ),
            ),
            visit_ignored,
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AtHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle for AtHandle {
    type View = AtView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::At)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::At
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        tree.collect_nodes(
            self.0,
            [NodeKind::Terminal(TerminalKind::At)],
            |[at], visit_ignored| Ok(visit(AtView { at: At(at) }, visit_ignored)),
            visit_ignored,
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AtView {
    pub at: At,
}
impl AtView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BeginHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle for BeginHandle {
    type View = BeginView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::Begin)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Begin
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        tree.collect_nodes(
            self.0,
            [NodeKind::Terminal(TerminalKind::LBrace)],
            |[l_brace], visit_ignored| Ok(
                visit(
                    BeginView {
                        l_brace: LBrace(l_brace),
                    },
                    visit_ignored,
                ),
            ),
            visit_ignored,
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BeginView {
    pub l_brace: LBrace,
}
impl BeginView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BindHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle for BindHandle {
    type View = BindView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::Bind)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Bind
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        tree.collect_nodes(
            self.0,
            [NodeKind::Terminal(TerminalKind::Bind)],
            |[bind], visit_ignored| Ok(
                visit(BindView { bind: Bind(bind) }, visit_ignored),
            ),
            visit_ignored,
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BindView {
    pub bind: Bind,
}
impl BindView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BindingHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle for BindingHandle {
    type View = BindingView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::Binding)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Binding
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::Keys),
                NodeKind::NonTerminal(NonTerminalKind::BindingRhs),
            ],
            |[keys, binding_rhs], visit_ignored| Ok(
                visit(
                    BindingView {
                        keys: KeysHandle(keys),
                        binding_rhs: BindingRhsHandle(binding_rhs),
                    },
                    visit_ignored,
                ),
            ),
            visit_ignored,
        )
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
impl NonTerminalHandle for BindingRhsHandle {
    type View = BindingRhsView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::BindingRhs)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::BindingRhs
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
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
            _ => {
                return Err(ViewConstructionError::UnexpectedNode {
                    node: child,
                    data: child_data,
                    expected_kind: child_data.node_kind(),
                });
            }
        };
        let (result, _visit) = visit(variant, visit_ignored);
        if let Some(child) = children.next() {
            return Err(ViewConstructionError::UnexpectedExtraNode {
                node: child,
            });
        }
        Ok(result)
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
impl NonTerminalHandle for BooleanHandle {
    type View = BooleanView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::Boolean)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Boolean
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
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
            _ => {
                return Err(ViewConstructionError::UnexpectedNode {
                    node: child,
                    data: child_data,
                    expected_kind: child_data.node_kind(),
                });
            }
        };
        let (result, _visit) = visit(variant, visit_ignored);
        if let Some(child) = children.next() {
            return Err(ViewConstructionError::UnexpectedExtraNode {
                node: child,
            });
        }
        Ok(result)
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
impl NonTerminalHandle for CodeHandle {
    type View = CodeView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::Code)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Code
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        tree.collect_nodes(
            self.0,
            [NodeKind::Terminal(TerminalKind::Code)],
            |[code], visit_ignored| Ok(
                visit(CodeView { code: Code(code) }, visit_ignored),
            ),
            visit_ignored,
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodeView {
    pub code: Code,
}
impl CodeView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CodeBlockHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle for CodeBlockHandle {
    type View = CodeBlockView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::CodeBlock)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::CodeBlock
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        tree.collect_nodes(
            self.0,
            [NodeKind::Terminal(TerminalKind::CodeBlock)],
            |[code_block], visit_ignored| Ok(
                visit(
                    CodeBlockView {
                        code_block: CodeBlock(code_block),
                    },
                    visit_ignored,
                ),
            ),
            visit_ignored,
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodeBlockView {
    pub code_block: CodeBlock,
}
impl CodeBlockView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommaHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle for CommaHandle {
    type View = CommaView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::Comma)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Comma
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        tree.collect_nodes(
            self.0,
            [NodeKind::Terminal(TerminalKind::Comma)],
            |[comma], visit_ignored| Ok(
                visit(CommaView { comma: Comma(comma) }, visit_ignored),
            ),
            visit_ignored,
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommaView {
    pub comma: Comma,
}
impl CommaView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ContinueHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle for ContinueHandle {
    type View = ContinueView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::Continue)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Continue
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        tree.collect_nodes(
            self.0,
            [NodeKind::Terminal(TerminalKind::Esc)],
            |[esc], visit_ignored| Ok(
                visit(ContinueView { esc: Esc(esc) }, visit_ignored),
            ),
            visit_ignored,
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContinueView {
    pub esc: Esc,
}
impl ContinueView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DotHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle for DotHandle {
    type View = DotView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::Dot)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Dot
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        tree.collect_nodes(
            self.0,
            [NodeKind::Terminal(TerminalKind::Dot)],
            |[dot], visit_ignored| Ok(visit(DotView { dot: Dot(dot) }, visit_ignored)),
            visit_ignored,
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DotView {
    pub dot: Dot,
}
impl DotView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EndHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle for EndHandle {
    type View = EndView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::End)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::End
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        tree.collect_nodes(
            self.0,
            [NodeKind::Terminal(TerminalKind::RBrace)],
            |[r_brace], visit_ignored| Ok(
                visit(
                    EndView {
                        r_brace: RBrace(r_brace),
                    },
                    visit_ignored,
                ),
            ),
            visit_ignored,
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EndView {
    pub r_brace: RBrace,
}
impl EndView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExtHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle for ExtHandle {
    type View = ExtView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::Ext)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Ext
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        tree.collect_nodes(
            self.0,
            [NodeKind::Terminal(TerminalKind::Dollar)],
            |[dollar], visit_ignored| Ok(
                visit(ExtView { dollar: Dollar(dollar) }, visit_ignored),
            ),
            visit_ignored,
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtView {
    pub dollar: Dollar,
}
impl ExtView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExtensionNameSpaceHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle for ExtensionNameSpaceHandle {
    type View = ExtensionNameSpaceView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::ExtensionNameSpace)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::ExtensionNameSpace
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::Ext),
                NodeKind::NonTerminal(NonTerminalKind::Ident),
            ],
            |[ext, ident], visit_ignored| Ok(
                visit(
                    ExtensionNameSpaceView {
                        ext: ExtHandle(ext),
                        ident: IdentHandle(ident),
                    },
                    visit_ignored,
                ),
            ),
            visit_ignored,
        )
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
impl NonTerminalHandle for FalseHandle {
    type View = FalseView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::False)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::False
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        tree.collect_nodes(
            self.0,
            [NodeKind::Terminal(TerminalKind::False)],
            |[r#false], visit_ignored| Ok(
                visit(
                    FalseView {
                        r#false: False(r#false),
                    },
                    visit_ignored,
                ),
            ),
            visit_ignored,
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FalseView {
    pub r#false: False,
}
impl FalseView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HoleHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle for HoleHandle {
    type View = HoleView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::Hole)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Hole
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        tree.collect_nodes(
            self.0,
            [NodeKind::Terminal(TerminalKind::Hole)],
            |[hole], visit_ignored| Ok(
                visit(HoleView { hole: Hole(hole) }, visit_ignored),
            ),
            visit_ignored,
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HoleView {
    pub hole: Hole,
}
impl HoleView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IdentHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle for IdentHandle {
    type View = IdentView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::Ident)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Ident
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        tree.collect_nodes(
            self.0,
            [NodeKind::Terminal(TerminalKind::Ident)],
            |[ident], visit_ignored| Ok(
                visit(IdentView { ident: Ident(ident) }, visit_ignored),
            ),
            visit_ignored,
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IdentView {
    pub ident: Ident,
}
impl IdentView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InStrHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle for InStrHandle {
    type View = InStrView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::InStr)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::InStr
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        tree.collect_nodes(
            self.0,
            [NodeKind::Terminal(TerminalKind::InStr)],
            |[in_str], visit_ignored| Ok(
                visit(InStrView { in_str: InStr(in_str) }, visit_ignored),
            ),
            visit_ignored,
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InStrView {
    pub in_str: InStr,
}
impl InStrView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IntegerHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle for IntegerHandle {
    type View = IntegerView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::Integer)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Integer
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        tree.collect_nodes(
            self.0,
            [NodeKind::Terminal(TerminalKind::Integer)],
            |[integer], visit_ignored| Ok(
                visit(
                    IntegerView {
                        integer: Integer(integer),
                    },
                    visit_ignored,
                ),
            ),
            visit_ignored,
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntegerView {
    pub integer: Integer,
}
impl IntegerView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeyHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle for KeyHandle {
    type View = KeyView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::Key)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Key
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::KeyBase),
                NodeKind::NonTerminal(NonTerminalKind::KeyOpt),
            ],
            |[key_base, key_opt], visit_ignored| Ok(
                visit(
                    KeyView {
                        key_base: KeyBaseHandle(key_base),
                        key_opt: KeyOptHandle(key_opt),
                    },
                    visit_ignored,
                ),
            ),
            visit_ignored,
        )
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
impl NonTerminalHandle for KeyBaseHandle {
    type View = KeyBaseView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::KeyBase)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::KeyBase
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
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
            _ => {
                return Err(ViewConstructionError::UnexpectedNode {
                    node: child,
                    data: child_data,
                    expected_kind: child_data.node_kind(),
                });
            }
        };
        let (result, _visit) = visit(variant, visit_ignored);
        if let Some(child) = children.next() {
            return Err(ViewConstructionError::UnexpectedExtraNode {
                node: child,
            });
        }
        Ok(result)
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
impl NonTerminalHandle for KeyOptHandle {
    type View = Option<ArrayMarkerHandle>;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::KeyOpt)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::KeyOpt
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        if tree.has_no_children(self.0) {
            return Ok(visit(None, visit_ignored).0);
        }
        tree.collect_nodes(
            self.0,
            [NodeKind::NonTerminal(NonTerminalKind::ArrayMarker)],
            |[child], visit_ignored| Ok(
                visit(
                    Some(ArrayMarkerHandle::new_with_visit(child, tree, visit_ignored)?),
                    visit_ignored,
                ),
            ),
            visit_ignored,
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeysHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle for KeysHandle {
    type View = KeysView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::Keys)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Keys
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::Key),
                NodeKind::NonTerminal(NonTerminalKind::KeysList),
            ],
            |[key, keys_list], visit_ignored| Ok(
                visit(
                    KeysView {
                        key: KeyHandle(key),
                        keys_list: KeysListHandle(keys_list),
                    },
                    visit_ignored,
                ),
            ),
            visit_ignored,
        )
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
impl NonTerminalHandle for KeysListHandle {
    type View = Option<KeysListView>;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::KeysList)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::KeysList
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        if tree.has_no_children(self.0) {
            return Ok(visit(None, visit_ignored).0);
        }
        tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::Dot),
                NodeKind::NonTerminal(NonTerminalKind::Key),
                NodeKind::NonTerminal(NonTerminalKind::KeysList),
            ],
            |[dot, key, keys_list], visit_ignored| Ok(
                visit(
                    Some(KeysListView {
                        dot: DotHandle(dot),
                        key: KeyHandle(key),
                        keys_list: KeysListHandle(keys_list),
                    }),
                    visit_ignored,
                ),
            ),
            visit_ignored,
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeysListView {
    pub dot: DotHandle,
    pub key: KeyHandle,
    pub keys_list: KeysListHandle,
}
impl<F: CstFacade> RecursiveView<F> for KeysListView {
    type Item = KeysListItem;
    fn get_all_with_visit<E>(
        &self,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Vec<Self::Item>, CstConstructError<E>> {
        let mut items = Vec::new();
        let mut current_view = Some(*self);
        while let Some(item) = current_view {
            let Self { dot, key, .. } = item;
            items.push(KeysListItem { dot, key });
            item.keys_list
                .get_view_with_visit(
                    tree,
                    |view, visit_ignored| {
                        current_view = view;
                        ((), visit_ignored)
                    },
                    visit_ignored,
                )?;
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
impl NonTerminalHandle for NamedCodeHandle {
    type View = NamedCodeView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::NamedCode)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::NamedCode
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        tree.collect_nodes(
            self.0,
            [NodeKind::Terminal(TerminalKind::NamedCode)],
            |[named_code], visit_ignored| Ok(
                visit(
                    NamedCodeView {
                        named_code: NamedCode(named_code),
                    },
                    visit_ignored,
                ),
            ),
            visit_ignored,
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NamedCodeView {
    pub named_code: NamedCode,
}
impl NamedCodeView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NewlineHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle for NewlineHandle {
    type View = NewlineView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::Newline)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Newline
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        tree.collect_nodes(
            self.0,
            [NodeKind::Terminal(TerminalKind::Newline)],
            |[newline], visit_ignored| Ok(
                visit(
                    NewlineView {
                        newline: Newline(newline),
                    },
                    visit_ignored,
                ),
            ),
            visit_ignored,
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NewlineView {
    pub newline: Newline,
}
impl NewlineView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NullHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle for NullHandle {
    type View = NullView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::Null)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Null
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        tree.collect_nodes(
            self.0,
            [NodeKind::Terminal(TerminalKind::Null)],
            |[null], visit_ignored| Ok(
                visit(NullView { null: Null(null) }, visit_ignored),
            ),
            visit_ignored,
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NullView {
    pub null: Null,
}
impl NullView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle for ObjectHandle {
    type View = ObjectView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::Object)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Object
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::Begin),
                NodeKind::NonTerminal(NonTerminalKind::ObjectList),
                NodeKind::NonTerminal(NonTerminalKind::End),
            ],
            |[begin, object_list, end], visit_ignored| Ok(
                visit(
                    ObjectView {
                        begin: BeginHandle(begin),
                        object_list: ObjectListHandle(object_list),
                        end: EndHandle(end),
                    },
                    visit_ignored,
                ),
            ),
            visit_ignored,
        )
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
impl NonTerminalHandle for ObjectListHandle {
    type View = Option<ObjectListView>;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::ObjectList)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::ObjectList
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        if tree.has_no_children(self.0) {
            return Ok(visit(None, visit_ignored).0);
        }
        tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::Key),
                NodeKind::NonTerminal(NonTerminalKind::Bind),
                NodeKind::NonTerminal(NonTerminalKind::Value),
                NodeKind::NonTerminal(NonTerminalKind::ObjectOpt),
                NodeKind::NonTerminal(NonTerminalKind::ObjectList),
            ],
            |[key, bind, value, object_opt, object_list], visit_ignored| Ok(
                visit(
                    Some(ObjectListView {
                        key: KeyHandle(key),
                        bind: BindHandle(bind),
                        value: ValueHandle(value),
                        object_opt: ObjectOptHandle(object_opt),
                        object_list: ObjectListHandle(object_list),
                    }),
                    visit_ignored,
                ),
            ),
            visit_ignored,
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
impl<F: CstFacade> RecursiveView<F> for ObjectListView {
    type Item = ObjectListItem;
    fn get_all_with_visit<E>(
        &self,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Vec<Self::Item>, CstConstructError<E>> {
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
            item.object_list
                .get_view_with_visit(
                    tree,
                    |view, visit_ignored| {
                        current_view = view;
                        ((), visit_ignored)
                    },
                    visit_ignored,
                )?;
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
impl NonTerminalHandle for ObjectOptHandle {
    type View = Option<CommaHandle>;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::ObjectOpt)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::ObjectOpt
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        if tree.has_no_children(self.0) {
            return Ok(visit(None, visit_ignored).0);
        }
        tree.collect_nodes(
            self.0,
            [NodeKind::NonTerminal(NonTerminalKind::Comma)],
            |[child], visit_ignored| Ok(
                visit(
                    Some(CommaHandle::new_with_visit(child, tree, visit_ignored)?),
                    visit_ignored,
                ),
            ),
            visit_ignored,
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QuoteHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle for QuoteHandle {
    type View = QuoteView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::Quote)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Quote
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        tree.collect_nodes(
            self.0,
            [NodeKind::Terminal(TerminalKind::Quote)],
            |[quote], visit_ignored| Ok(
                visit(QuoteView { quote: Quote(quote) }, visit_ignored),
            ),
            visit_ignored,
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuoteView {
    pub quote: Quote,
}
impl QuoteView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SectionHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle for SectionHandle {
    type View = SectionView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::Section)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Section
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::At),
                NodeKind::NonTerminal(NonTerminalKind::Keys),
                NodeKind::NonTerminal(NonTerminalKind::SectionBody),
            ],
            |[at, keys, section_body], visit_ignored| Ok(
                visit(
                    SectionView {
                        at: AtHandle(at),
                        keys: KeysHandle(keys),
                        section_body: SectionBodyHandle(section_body),
                    },
                    visit_ignored,
                ),
            ),
            visit_ignored,
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SectionView {
    pub at: AtHandle,
    pub keys: KeysHandle,
    pub section_body: SectionBodyHandle,
}
impl SectionView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SectionBindingHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle for SectionBindingHandle {
    type View = SectionBindingView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::SectionBinding)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::SectionBinding
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::Begin),
                NodeKind::NonTerminal(NonTerminalKind::Swon),
                NodeKind::NonTerminal(NonTerminalKind::End),
            ],
            |[begin, swon, end], visit_ignored| Ok(
                visit(
                    SectionBindingView {
                        begin: BeginHandle(begin),
                        swon: SwonHandle(swon),
                        end: EndHandle(end),
                    },
                    visit_ignored,
                ),
            ),
            visit_ignored,
        )
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
pub struct SectionBodyHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle for SectionBodyHandle {
    type View = SectionBodyView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::SectionBody)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::SectionBody
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
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
            NodeKind::NonTerminal(NonTerminalKind::SectionBodyList) => {
                SectionBodyView::SectionBodyList(SectionBodyListHandle(child))
            }
            NodeKind::NonTerminal(NonTerminalKind::SectionBinding) => {
                SectionBodyView::SectionBinding(SectionBindingHandle(child))
            }
            _ => {
                return Err(ViewConstructionError::UnexpectedNode {
                    node: child,
                    data: child_data,
                    expected_kind: child_data.node_kind(),
                });
            }
        };
        let (result, _visit) = visit(variant, visit_ignored);
        if let Some(child) = children.next() {
            return Err(ViewConstructionError::UnexpectedExtraNode {
                node: child,
            });
        }
        Ok(result)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionBodyView {
    SectionBodyList(SectionBodyListHandle),
    SectionBinding(SectionBindingHandle),
}
impl SectionBodyView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SectionBodyListHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle for SectionBodyListHandle {
    type View = Option<SectionBodyListView>;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::SectionBodyList)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::SectionBodyList
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        if tree.has_no_children(self.0) {
            return Ok(visit(None, visit_ignored).0);
        }
        tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::Binding),
                NodeKind::NonTerminal(NonTerminalKind::SectionBodyList),
            ],
            |[binding, section_body_list], visit_ignored| Ok(
                visit(
                    Some(SectionBodyListView {
                        binding: BindingHandle(binding),
                        section_body_list: SectionBodyListHandle(section_body_list),
                    }),
                    visit_ignored,
                ),
            ),
            visit_ignored,
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SectionBodyListView {
    pub binding: BindingHandle,
    pub section_body_list: SectionBodyListHandle,
}
impl<F: CstFacade> RecursiveView<F> for SectionBodyListView {
    type Item = BindingHandle;
    fn get_all_with_visit<E>(
        &self,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Vec<Self::Item>, CstConstructError<E>> {
        let mut items = Vec::new();
        let mut current_view = Some(*self);
        while let Some(item) = current_view {
            let Self { binding, .. } = item;
            items.push(binding);
            item.section_body_list
                .get_view_with_visit(
                    tree,
                    |view, visit_ignored| {
                        current_view = view;
                        ((), visit_ignored)
                    },
                    visit_ignored,
                )?;
        }
        Ok(items)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StrHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle for StrHandle {
    type View = StrView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::Str)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Str
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::Quote),
                NodeKind::NonTerminal(NonTerminalKind::InStr),
                NodeKind::NonTerminal(NonTerminalKind::Quote),
            ],
            |[quote, in_str, quote2], visit_ignored| Ok(
                visit(
                    StrView {
                        quote: QuoteHandle(quote),
                        in_str: InStrHandle(in_str),
                        quote2: QuoteHandle(quote2),
                    },
                    visit_ignored,
                ),
            ),
            visit_ignored,
        )
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
impl NonTerminalHandle for StrContinuesHandle {
    type View = StrContinuesView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::StrContinues)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::StrContinues
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::Str),
                NodeKind::NonTerminal(NonTerminalKind::StrContinuesList),
            ],
            |[str, str_continues_list], visit_ignored| Ok(
                visit(
                    StrContinuesView {
                        str: StrHandle(str),
                        str_continues_list: StrContinuesListHandle(str_continues_list),
                    },
                    visit_ignored,
                ),
            ),
            visit_ignored,
        )
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
impl NonTerminalHandle for StrContinuesListHandle {
    type View = Option<StrContinuesListView>;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::StrContinuesList)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::StrContinuesList
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        if tree.has_no_children(self.0) {
            return Ok(visit(None, visit_ignored).0);
        }
        tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::Continue),
                NodeKind::NonTerminal(NonTerminalKind::Str),
                NodeKind::NonTerminal(NonTerminalKind::StrContinuesList),
            ],
            |[r#continue, str, str_continues_list], visit_ignored| Ok(
                visit(
                    Some(StrContinuesListView {
                        r#continue: ContinueHandle(r#continue),
                        str: StrHandle(str),
                        str_continues_list: StrContinuesListHandle(str_continues_list),
                    }),
                    visit_ignored,
                ),
            ),
            visit_ignored,
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StrContinuesListView {
    pub r#continue: ContinueHandle,
    pub str: StrHandle,
    pub str_continues_list: StrContinuesListHandle,
}
impl<F: CstFacade> RecursiveView<F> for StrContinuesListView {
    type Item = StrContinuesListItem;
    fn get_all_with_visit<E>(
        &self,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Vec<Self::Item>, CstConstructError<E>> {
        let mut items = Vec::new();
        let mut current_view = Some(*self);
        while let Some(item) = current_view {
            let Self { r#continue, str, .. } = item;
            items
                .push(StrContinuesListItem {
                    r#continue,
                    str,
                });
            item.str_continues_list
                .get_view_with_visit(
                    tree,
                    |view, visit_ignored| {
                        current_view = view;
                        ((), visit_ignored)
                    },
                    visit_ignored,
                )?;
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
impl NonTerminalHandle for SwonHandle {
    type View = SwonView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::Swon)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Swon
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::SwonList),
                NodeKind::NonTerminal(NonTerminalKind::SwonList0),
            ],
            |[swon_bindings, swon_sections], visit_ignored| Ok(
                visit(
                    SwonView {
                        swon_bindings: SwonBindingsHandle(swon_bindings),
                        swon_sections: SwonSectionsHandle(swon_sections),
                    },
                    visit_ignored,
                ),
            ),
            visit_ignored,
        )
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
impl NonTerminalHandle for SwonBindingsHandle {
    type View = Option<SwonBindingsView>;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::SwonList)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::SwonList
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        if tree.has_no_children(self.0) {
            return Ok(visit(None, visit_ignored).0);
        }
        tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::Binding),
                NodeKind::NonTerminal(NonTerminalKind::SwonList),
            ],
            |[binding, swon_bindings], visit_ignored| Ok(
                visit(
                    Some(SwonBindingsView {
                        binding: BindingHandle(binding),
                        swon_bindings: SwonBindingsHandle(swon_bindings),
                    }),
                    visit_ignored,
                ),
            ),
            visit_ignored,
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SwonBindingsView {
    pub binding: BindingHandle,
    pub swon_bindings: SwonBindingsHandle,
}
impl<F: CstFacade> RecursiveView<F> for SwonBindingsView {
    type Item = BindingHandle;
    fn get_all_with_visit<E>(
        &self,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Vec<Self::Item>, CstConstructError<E>> {
        let mut items = Vec::new();
        let mut current_view = Some(*self);
        while let Some(item) = current_view {
            let Self { binding, .. } = item;
            items.push(binding);
            item.swon_bindings
                .get_view_with_visit(
                    tree,
                    |view, visit_ignored| {
                        current_view = view;
                        ((), visit_ignored)
                    },
                    visit_ignored,
                )?;
        }
        Ok(items)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SwonSectionsHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle for SwonSectionsHandle {
    type View = Option<SwonSectionsView>;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::SwonList0)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::SwonList0
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        if tree.has_no_children(self.0) {
            return Ok(visit(None, visit_ignored).0);
        }
        tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::Section),
                NodeKind::NonTerminal(NonTerminalKind::SwonList0),
            ],
            |[section, swon_sections], visit_ignored| Ok(
                visit(
                    Some(SwonSectionsView {
                        section: SectionHandle(section),
                        swon_sections: SwonSectionsHandle(swon_sections),
                    }),
                    visit_ignored,
                ),
            ),
            visit_ignored,
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SwonSectionsView {
    pub section: SectionHandle,
    pub swon_sections: SwonSectionsHandle,
}
impl<F: CstFacade> RecursiveView<F> for SwonSectionsView {
    type Item = SectionHandle;
    fn get_all_with_visit<E>(
        &self,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Vec<Self::Item>, CstConstructError<E>> {
        let mut items = Vec::new();
        let mut current_view = Some(*self);
        while let Some(item) = current_view {
            let Self { section, .. } = item;
            items.push(section);
            item.swon_sections
                .get_view_with_visit(
                    tree,
                    |view, visit_ignored| {
                        current_view = view;
                        ((), visit_ignored)
                    },
                    visit_ignored,
                )?;
        }
        Ok(items)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle for TextHandle {
    type View = TextView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::Text)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Text
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        tree.collect_nodes(
            self.0,
            [NodeKind::Terminal(TerminalKind::Text)],
            |[text], visit_ignored| Ok(
                visit(TextView { text: Text(text) }, visit_ignored),
            ),
            visit_ignored,
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextView {
    pub text: Text,
}
impl TextView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextBindingHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle for TextBindingHandle {
    type View = TextBindingView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::TextBinding)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::TextBinding
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::TextStart),
                NodeKind::NonTerminal(NonTerminalKind::TextBindingOpt),
                NodeKind::NonTerminal(NonTerminalKind::Text),
                NodeKind::NonTerminal(NonTerminalKind::Newline),
            ],
            |[text_start, text_binding_opt, text, newline], visit_ignored| Ok(
                visit(
                    TextBindingView {
                        text_start: TextStartHandle(text_start),
                        text_binding_opt: TextBindingOptHandle(text_binding_opt),
                        text: TextHandle(text),
                        newline: NewlineHandle(newline),
                    },
                    visit_ignored,
                ),
            ),
            visit_ignored,
        )
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
impl NonTerminalHandle for TextBindingOptHandle {
    type View = Option<WsHandle>;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::TextBindingOpt)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::TextBindingOpt
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        if tree.has_no_children(self.0) {
            return Ok(visit(None, visit_ignored).0);
        }
        tree.collect_nodes(
            self.0,
            [NodeKind::NonTerminal(NonTerminalKind::Ws)],
            |[child], visit_ignored| Ok(
                visit(
                    Some(WsHandle::new_with_visit(child, tree, visit_ignored)?),
                    visit_ignored,
                ),
            ),
            visit_ignored,
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextStartHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle for TextStartHandle {
    type View = TextStartView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::TextStart)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::TextStart
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        tree.collect_nodes(
            self.0,
            [NodeKind::Terminal(TerminalKind::TextStart)],
            |[text_start], visit_ignored| Ok(
                visit(
                    TextStartView {
                        text_start: TextStart(text_start),
                    },
                    visit_ignored,
                ),
            ),
            visit_ignored,
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextStartView {
    pub text_start: TextStart,
}
impl TextStartView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TrueHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle for TrueHandle {
    type View = TrueView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::True)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::True
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        tree.collect_nodes(
            self.0,
            [NodeKind::Terminal(TerminalKind::True)],
            |[r#true], visit_ignored| Ok(
                visit(TrueView { r#true: True(r#true) }, visit_ignored),
            ),
            visit_ignored,
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrueView {
    pub r#true: True,
}
impl TrueView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypedQuoteHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle for TypedQuoteHandle {
    type View = TypedQuoteView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::TypedQuote)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::TypedQuote
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        tree.collect_nodes(
            self.0,
            [NodeKind::Terminal(TerminalKind::TypedQuote)],
            |[typed_quote], visit_ignored| Ok(
                visit(
                    TypedQuoteView {
                        typed_quote: TypedQuote(typed_quote),
                    },
                    visit_ignored,
                ),
            ),
            visit_ignored,
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypedQuoteView {
    pub typed_quote: TypedQuote,
}
impl TypedQuoteView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypedStrHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle for TypedStrHandle {
    type View = TypedStrView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::TypedStr)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::TypedStr
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::TypedQuote),
                NodeKind::NonTerminal(NonTerminalKind::InStr),
                NodeKind::NonTerminal(NonTerminalKind::Quote),
            ],
            |[typed_quote, in_str, quote], visit_ignored| Ok(
                visit(
                    TypedStrView {
                        typed_quote: TypedQuoteHandle(typed_quote),
                        in_str: InStrHandle(in_str),
                        quote: QuoteHandle(quote),
                    },
                    visit_ignored,
                ),
            ),
            visit_ignored,
        )
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
impl NonTerminalHandle for ValueHandle {
    type View = ValueView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::Value)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Value
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
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
            NodeKind::NonTerminal(NonTerminalKind::CodeBlock) => {
                ValueView::CodeBlock(CodeBlockHandle(child))
            }
            NodeKind::NonTerminal(NonTerminalKind::NamedCode) => {
                ValueView::NamedCode(NamedCodeHandle(child))
            }
            NodeKind::NonTerminal(NonTerminalKind::Code) => {
                ValueView::Code(CodeHandle(child))
            }
            _ => {
                return Err(ViewConstructionError::UnexpectedNode {
                    node: child,
                    data: child_data,
                    expected_kind: child_data.node_kind(),
                });
            }
        };
        let (result, _visit) = visit(variant, visit_ignored);
        if let Some(child) = children.next() {
            return Err(ViewConstructionError::UnexpectedExtraNode {
                node: child,
            });
        }
        Ok(result)
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
    CodeBlock(CodeBlockHandle),
    NamedCode(NamedCodeHandle),
    Code(CodeHandle),
}
impl ValueView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ValueBindingHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle for ValueBindingHandle {
    type View = ValueBindingView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::ValueBinding)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::ValueBinding
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::Bind),
                NodeKind::NonTerminal(NonTerminalKind::Value),
            ],
            |[bind, value], visit_ignored| Ok(
                visit(
                    ValueBindingView {
                        bind: BindHandle(bind),
                        value: ValueHandle(value),
                    },
                    visit_ignored,
                ),
            ),
            visit_ignored,
        )
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
impl NonTerminalHandle for WsHandle {
    type View = WsView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::Ws)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Ws
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        tree.collect_nodes(
            self.0,
            [NodeKind::Terminal(TerminalKind::Ws)],
            |[ws], visit_ignored| Ok(visit(WsView { ws: Ws(ws) }, visit_ignored)),
            visit_ignored,
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WsView {
    pub ws: Ws,
}
impl WsView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RootHandle(pub(crate) super::tree::CstNodeId);
impl NonTerminalHandle for RootHandle {
    type View = RootView;
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn new_with_visit<F: CstFacade, E>(
        index: CstNodeId,
        tree: &F,
        visit_ignored: &mut impl BuiltinTerminalVisitor<E, F>,
    ) -> Result<Self, CstConstructError<E>> {
        tree.collect_nodes(
            index,
            [NodeKind::NonTerminal(NonTerminalKind::Root)],
            |[index], visit| Ok((Self(index), visit)),
            visit_ignored,
        )
    }
    fn kind(&self) -> NonTerminalKind {
        NonTerminalKind::Root
    }
    fn get_view_with_visit<'v, F: CstFacade, V: BuiltinTerminalVisitor<E, F>, O, E>(
        &self,
        tree: &F,
        mut visit: impl FnMut(Self::View, &'v mut V) -> (O, &'v mut V),
        visit_ignored: &'v mut V,
    ) -> Result<O, CstConstructError<E>> {
        tree.collect_nodes(
            self.0,
            [NodeKind::NonTerminal(NonTerminalKind::Swon)],
            |[swon], visit_ignored| Ok(
                visit(RootView { swon: SwonHandle(swon) }, visit_ignored),
            ),
            visit_ignored,
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RootView {
    pub swon: SwonHandle,
}
impl RootView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NewLine(pub(crate) super::tree::CstNodeId);
impl TerminalHandle for NewLine {
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn kind(&self) -> TerminalKind {
        TerminalKind::NewLine
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Whitespace(pub(crate) super::tree::CstNodeId);
impl TerminalHandle for Whitespace {
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn kind(&self) -> TerminalKind {
        TerminalKind::Whitespace
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LineComment(pub(crate) super::tree::CstNodeId);
impl TerminalHandle for LineComment {
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn kind(&self) -> TerminalKind {
        TerminalKind::LineComment
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockComment(pub(crate) super::tree::CstNodeId);
impl TerminalHandle for BlockComment {
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn kind(&self) -> TerminalKind {
        TerminalKind::BlockComment
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Integer(pub(crate) super::tree::CstNodeId);
impl TerminalHandle for Integer {
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn kind(&self) -> TerminalKind {
        TerminalKind::Integer
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct True(pub(crate) super::tree::CstNodeId);
impl TerminalHandle for True {
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn kind(&self) -> TerminalKind {
        TerminalKind::True
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct False(pub(crate) super::tree::CstNodeId);
impl TerminalHandle for False {
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn kind(&self) -> TerminalKind {
        TerminalKind::False
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Null(pub(crate) super::tree::CstNodeId);
impl TerminalHandle for Null {
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn kind(&self) -> TerminalKind {
        TerminalKind::Null
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Hole(pub(crate) super::tree::CstNodeId);
impl TerminalHandle for Hole {
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn kind(&self) -> TerminalKind {
        TerminalKind::Hole
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Quote(pub(crate) super::tree::CstNodeId);
impl TerminalHandle for Quote {
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn kind(&self) -> TerminalKind {
        TerminalKind::Quote
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypedQuote(pub(crate) super::tree::CstNodeId);
impl TerminalHandle for TypedQuote {
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn kind(&self) -> TerminalKind {
        TerminalKind::TypedQuote
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InStr(pub(crate) super::tree::CstNodeId);
impl TerminalHandle for InStr {
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn kind(&self) -> TerminalKind {
        TerminalKind::InStr
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Text(pub(crate) super::tree::CstNodeId);
impl TerminalHandle for Text {
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn kind(&self) -> TerminalKind {
        TerminalKind::Text
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CodeBlock(pub(crate) super::tree::CstNodeId);
impl TerminalHandle for CodeBlock {
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn kind(&self) -> TerminalKind {
        TerminalKind::CodeBlock
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NamedCode(pub(crate) super::tree::CstNodeId);
impl TerminalHandle for NamedCode {
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn kind(&self) -> TerminalKind {
        TerminalKind::NamedCode
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Code(pub(crate) super::tree::CstNodeId);
impl TerminalHandle for Code {
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn kind(&self) -> TerminalKind {
        TerminalKind::Code
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Newline(pub(crate) super::tree::CstNodeId);
impl TerminalHandle for Newline {
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn kind(&self) -> TerminalKind {
        TerminalKind::Newline
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ws(pub(crate) super::tree::CstNodeId);
impl TerminalHandle for Ws {
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn kind(&self) -> TerminalKind {
        TerminalKind::Ws
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct At(pub(crate) super::tree::CstNodeId);
impl TerminalHandle for At {
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn kind(&self) -> TerminalKind {
        TerminalKind::At
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Dollar(pub(crate) super::tree::CstNodeId);
impl TerminalHandle for Dollar {
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn kind(&self) -> TerminalKind {
        TerminalKind::Dollar
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Dot(pub(crate) super::tree::CstNodeId);
impl TerminalHandle for Dot {
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn kind(&self) -> TerminalKind {
        TerminalKind::Dot
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LBrace(pub(crate) super::tree::CstNodeId);
impl TerminalHandle for LBrace {
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn kind(&self) -> TerminalKind {
        TerminalKind::LBrace
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RBrace(pub(crate) super::tree::CstNodeId);
impl TerminalHandle for RBrace {
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn kind(&self) -> TerminalKind {
        TerminalKind::RBrace
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LBracket(pub(crate) super::tree::CstNodeId);
impl TerminalHandle for LBracket {
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn kind(&self) -> TerminalKind {
        TerminalKind::LBracket
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RBracket(pub(crate) super::tree::CstNodeId);
impl TerminalHandle for RBracket {
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn kind(&self) -> TerminalKind {
        TerminalKind::RBracket
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Bind(pub(crate) super::tree::CstNodeId);
impl TerminalHandle for Bind {
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn kind(&self) -> TerminalKind {
        TerminalKind::Bind
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Comma(pub(crate) super::tree::CstNodeId);
impl TerminalHandle for Comma {
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn kind(&self) -> TerminalKind {
        TerminalKind::Comma
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Esc(pub(crate) super::tree::CstNodeId);
impl TerminalHandle for Esc {
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn kind(&self) -> TerminalKind {
        TerminalKind::Esc
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextStart(pub(crate) super::tree::CstNodeId);
impl TerminalHandle for TextStart {
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn kind(&self) -> TerminalKind {
        TerminalKind::TextStart
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ident(pub(crate) super::tree::CstNodeId);
impl TerminalHandle for Ident {
    fn node_id(&self) -> CstNodeId {
        self.0
    }
    fn kind(&self) -> TerminalKind {
        TerminalKind::Ident
    }
}
