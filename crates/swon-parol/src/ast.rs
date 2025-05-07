use super::nodes::{NonTerminalKind, TerminalKind};
use super::tree::{CstNodeId, NonTerminalHandle, ViewConstructionError, assert_node_kind};
use crate::Cst;
use parol_runtime::parser::parse_tree_type::NodeKind;
pub struct ArrayHandle(super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for ArrayHandle {
    type View = ArrayView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Array))?;
        Ok(Self(index))
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [array_begin, array_list, array_end] = tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::ArrayBegin),
                NodeKind::NonTerminal(NonTerminalKind::ArrayList),
                NodeKind::NonTerminal(NonTerminalKind::ArrayEnd),
            ],
        )?;
        Ok(ArrayView {
            array_begin: ArrayBeginHandle(array_begin),
            array_list: ArrayListHandle(array_list),
            array_end: ArrayEndHandle(array_end),
        })
    }
}
pub struct ArrayView {
    pub array_begin: ArrayBeginHandle,
    pub array_list: ArrayListHandle,
    pub array_end: ArrayEndHandle,
}
impl ArrayView {}
pub struct ArrayBeginHandle(super::tree::CstNodeId);
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
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [l_bracket] =
            tree.collect_nodes(self.0, [NodeKind::Terminal(TerminalKind::LBracket)])?;
        Ok(ArrayBeginView {
            l_bracket: LBracket(l_bracket),
        })
    }
}
pub struct ArrayBeginView {
    pub l_bracket: LBracket,
}
impl ArrayBeginView {}
pub struct ArrayEndHandle(super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for ArrayEndHandle {
    type View = ArrayEndView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(
            index,
            kind,
            NodeKind::NonTerminal(NonTerminalKind::ArrayEnd),
        )?;
        Ok(Self(index))
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [r_bracket] =
            tree.collect_nodes(self.0, [NodeKind::Terminal(TerminalKind::RBracket)])?;
        Ok(ArrayEndView {
            r_bracket: RBracket(r_bracket),
        })
    }
}
pub struct ArrayEndView {
    pub r_bracket: RBracket,
}
impl ArrayEndView {}
pub struct ArrayListHandle(super::tree::CstNodeId);
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
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        if tree.has_no_children(self.0) {
            return Ok(None);
        }
        let [value, array_opt, array_list] = tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::Value),
                NodeKind::NonTerminal(NonTerminalKind::ArrayOpt),
                NodeKind::NonTerminal(NonTerminalKind::ArrayList),
            ],
        )?;
        Ok(Some(ArrayListView {
            value: ValueHandle(value),
            array_opt: ArrayOptHandle(array_opt),
            array_list: ArrayListHandle(array_list),
        }))
    }
}
pub struct ArrayListView {
    pub value: ValueHandle,
    pub array_opt: ArrayOptHandle,
    pub array_list: ArrayListHandle,
}
impl ArrayListView {}
pub struct ArrayMarkerHandle(super::tree::CstNodeId);
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
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [array_begin, array_marker_opt, array_end] = tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::ArrayBegin),
                NodeKind::NonTerminal(NonTerminalKind::ArrayMarkerOpt),
                NodeKind::NonTerminal(NonTerminalKind::ArrayEnd),
            ],
        )?;
        Ok(ArrayMarkerView {
            array_begin: ArrayBeginHandle(array_begin),
            array_marker_opt: ArrayMarkerOptHandle(array_marker_opt),
            array_end: ArrayEndHandle(array_end),
        })
    }
}
pub struct ArrayMarkerView {
    pub array_begin: ArrayBeginHandle,
    pub array_marker_opt: ArrayMarkerOptHandle,
    pub array_end: ArrayEndHandle,
}
impl ArrayMarkerView {}
pub struct ArrayMarkerOptHandle(super::tree::CstNodeId);
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
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        if tree.has_no_children(self.0) {
            return Ok(None);
        }
        let [child] =
            tree.collect_nodes(self.0, [NodeKind::NonTerminal(NonTerminalKind::Integer)])?;
        Ok(Some(IntegerHandle::new(
            child,
            NodeKind::NonTerminal(NonTerminalKind::Integer),
        )?))
    }
}
pub struct ArrayOptHandle(super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for ArrayOptHandle {
    type View = Option<CommaHandle>;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(
            index,
            kind,
            NodeKind::NonTerminal(NonTerminalKind::ArrayOpt),
        )?;
        Ok(Self(index))
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        if tree.has_no_children(self.0) {
            return Ok(None);
        }
        let [child] =
            tree.collect_nodes(self.0, [NodeKind::NonTerminal(NonTerminalKind::Comma)])?;
        Ok(Some(CommaHandle::new(
            child,
            NodeKind::NonTerminal(NonTerminalKind::Comma),
        )?))
    }
}
pub struct AtHandle(super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for AtHandle {
    type View = AtView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::At))?;
        Ok(Self(index))
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [at] = tree.collect_nodes(self.0, [NodeKind::Terminal(TerminalKind::At)])?;
        Ok(AtView { at: At(at) })
    }
}
pub struct AtView {
    pub at: At,
}
impl AtView {}
pub struct BeginHandle(super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for BeginHandle {
    type View = BeginView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Begin))?;
        Ok(Self(index))
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [l_brace] = tree.collect_nodes(self.0, [NodeKind::Terminal(TerminalKind::LBrace)])?;
        Ok(BeginView {
            l_brace: LBrace(l_brace),
        })
    }
}
pub struct BeginView {
    pub l_brace: LBrace,
}
impl BeginView {}
pub struct BindHandle(super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for BindHandle {
    type View = BindView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Bind))?;
        Ok(Self(index))
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [bind] = tree.collect_nodes(self.0, [NodeKind::Terminal(TerminalKind::Bind)])?;
        Ok(BindView { bind: Bind(bind) })
    }
}
pub struct BindView {
    pub bind: Bind,
}
impl BindView {}
pub struct BindingHandle(super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for BindingHandle {
    type View = BindingView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Binding))?;
        Ok(Self(index))
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [keys, bindings] = tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::Keys),
                NodeKind::NonTerminal(NonTerminalKind::Bindings),
            ],
        )?;
        Ok(BindingView {
            keys: KeysHandle(keys),
            bindings: BindingsHandle(bindings),
        })
    }
}
pub struct BindingView {
    pub keys: KeysHandle,
    pub bindings: BindingsHandle,
}
impl BindingView {}
pub struct BindingsHandle(super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for BindingsHandle {
    type View = BindingsView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(
            index,
            kind,
            NodeKind::NonTerminal(NonTerminalKind::Bindings),
        )?;
        Ok(Self(index))
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let mut children = tree.children(self.0);
        let Some(child) = children.next() else {
            return Err(ViewConstructionError::UnexpectedEndOfChildren { parent: self.0 });
        };
        let Some(child_data) = tree.node_data(child) else {
            return Err(ViewConstructionError::NodeIdNotFound { node: child });
        };
        let variant = match child_data.node_kind() {
            NodeKind::NonTerminal(NonTerminalKind::ValueBinding) => {
                BindingsView::ValueBinding(ValueBindingHandle(child))
            }
            NodeKind::NonTerminal(NonTerminalKind::SectionBinding) => {
                BindingsView::SectionBinding(SectionBindingHandle(child))
            }
            NodeKind::NonTerminal(NonTerminalKind::TextBinding) => {
                BindingsView::TextBinding(TextBindingHandle(child))
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
            return Err(ViewConstructionError::UnexpectedExtraNode { node: child });
        }
        Ok(variant)
    }
}
pub enum BindingsView {
    ValueBinding(ValueBindingHandle),
    SectionBinding(SectionBindingHandle),
    TextBinding(TextBindingHandle),
}
impl BindingsView {}
pub struct BooleanHandle(super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for BooleanHandle {
    type View = BooleanView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Boolean))?;
        Ok(Self(index))
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let mut children = tree.children(self.0);
        let Some(child) = children.next() else {
            return Err(ViewConstructionError::UnexpectedEndOfChildren { parent: self.0 });
        };
        let Some(child_data) = tree.node_data(child) else {
            return Err(ViewConstructionError::NodeIdNotFound { node: child });
        };
        let variant = match child_data.node_kind() {
            NodeKind::NonTerminal(NonTerminalKind::True) => BooleanView::True(TrueHandle(child)),
            NodeKind::NonTerminal(NonTerminalKind::False) => BooleanView::False(FalseHandle(child)),
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
            return Err(ViewConstructionError::UnexpectedExtraNode { node: child });
        }
        Ok(variant)
    }
}
pub enum BooleanView {
    True(TrueHandle),
    False(FalseHandle),
}
impl BooleanView {}
pub struct CodeHandle(super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for CodeHandle {
    type View = CodeView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Code))?;
        Ok(Self(index))
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [code] = tree.collect_nodes(self.0, [NodeKind::Terminal(TerminalKind::Code)])?;
        Ok(CodeView { code: Code(code) })
    }
}
pub struct CodeView {
    pub code: Code,
}
impl CodeView {}
pub struct CodeBlockHandle(super::tree::CstNodeId);
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
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [code_block_delimiter, code_block_tail_common] = tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::CodeBlockDelimiter),
                NodeKind::NonTerminal(NonTerminalKind::CodeBlockTailCommon),
            ],
        )?;
        Ok(CodeBlockView {
            code_block_delimiter: CodeBlockDelimiterHandle(code_block_delimiter),
            code_block_tail_common: CodeBlockTailCommonHandle(code_block_tail_common),
        })
    }
}
pub struct CodeBlockView {
    pub code_block_delimiter: CodeBlockDelimiterHandle,
    pub code_block_tail_common: CodeBlockTailCommonHandle,
}
impl CodeBlockView {}
pub struct CodeBlockDelimiterHandle(super::tree::CstNodeId);
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
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [code_block_delimiter] = tree.collect_nodes(
            self.0,
            [NodeKind::Terminal(TerminalKind::CodeBlockDelimiter)],
        )?;
        Ok(CodeBlockDelimiterView {
            code_block_delimiter: CodeBlockDelimiter(code_block_delimiter),
        })
    }
}
pub struct CodeBlockDelimiterView {
    pub code_block_delimiter: CodeBlockDelimiter,
}
impl CodeBlockDelimiterView {}
pub struct CodeBlockLineHandle(super::tree::CstNodeId);
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
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [code_block_line] =
            tree.collect_nodes(self.0, [NodeKind::Terminal(TerminalKind::CodeBlockLine)])?;
        Ok(CodeBlockLineView {
            code_block_line: CodeBlockLine(code_block_line),
        })
    }
}
pub struct CodeBlockLineView {
    pub code_block_line: CodeBlockLine,
}
impl CodeBlockLineView {}
pub struct CodeBlockTailCommonHandle(super::tree::CstNodeId);
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
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [
            newline,
            code_block_tail_common_list,
            code_block_tail_common_opt,
            code_block_delimiter,
        ] = tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::Newline),
                NodeKind::NonTerminal(NonTerminalKind::CodeBlockTailCommonList),
                NodeKind::NonTerminal(NonTerminalKind::CodeBlockTailCommonOpt),
                NodeKind::NonTerminal(NonTerminalKind::CodeBlockDelimiter),
            ],
        )?;
        Ok(CodeBlockTailCommonView {
            newline: NewlineHandle(newline),
            code_block_tail_common_list: CodeBlockTailCommonListHandle(code_block_tail_common_list),
            code_block_tail_common_opt: CodeBlockTailCommonOptHandle(code_block_tail_common_opt),
            code_block_delimiter: CodeBlockDelimiterHandle(code_block_delimiter),
        })
    }
}
pub struct CodeBlockTailCommonView {
    pub newline: NewlineHandle,
    pub code_block_tail_common_list: CodeBlockTailCommonListHandle,
    pub code_block_tail_common_opt: CodeBlockTailCommonOptHandle,
    pub code_block_delimiter: CodeBlockDelimiterHandle,
}
impl CodeBlockTailCommonView {}
pub struct CodeBlockTailCommonListHandle(super::tree::CstNodeId);
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
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        if tree.has_no_children(self.0) {
            return Ok(None);
        }
        let [code_block_line, newline, code_block_tail_common_list] = tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::CodeBlockLine),
                NodeKind::NonTerminal(NonTerminalKind::Newline),
                NodeKind::NonTerminal(NonTerminalKind::CodeBlockTailCommonList),
            ],
        )?;
        Ok(Some(CodeBlockTailCommonListView {
            code_block_line: CodeBlockLineHandle(code_block_line),
            newline: NewlineHandle(newline),
            code_block_tail_common_list: CodeBlockTailCommonListHandle(code_block_tail_common_list),
        }))
    }
}
pub struct CodeBlockTailCommonListView {
    pub code_block_line: CodeBlockLineHandle,
    pub newline: NewlineHandle,
    pub code_block_tail_common_list: CodeBlockTailCommonListHandle,
}
impl CodeBlockTailCommonListView {}
pub struct CodeBlockTailCommonOptHandle(super::tree::CstNodeId);
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
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        if tree.has_no_children(self.0) {
            return Ok(None);
        }
        let [child] = tree.collect_nodes(self.0, [NodeKind::NonTerminal(NonTerminalKind::Ws)])?;
        Ok(Some(WsHandle::new(
            child,
            NodeKind::NonTerminal(NonTerminalKind::Ws),
        )?))
    }
}
pub struct CommaHandle(super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for CommaHandle {
    type View = CommaView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Comma))?;
        Ok(Self(index))
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [comma] = tree.collect_nodes(self.0, [NodeKind::Terminal(TerminalKind::Comma)])?;
        Ok(CommaView {
            comma: Comma(comma),
        })
    }
}
pub struct CommaView {
    pub comma: Comma,
}
impl CommaView {}
pub struct ContinueHandle(super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for ContinueHandle {
    type View = ContinueView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(
            index,
            kind,
            NodeKind::NonTerminal(NonTerminalKind::Continue),
        )?;
        Ok(Self(index))
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [esc] = tree.collect_nodes(self.0, [NodeKind::Terminal(TerminalKind::Esc)])?;
        Ok(ContinueView { esc: Esc(esc) })
    }
}
pub struct ContinueView {
    pub esc: Esc,
}
impl ContinueView {}
pub struct DotHandle(super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for DotHandle {
    type View = DotView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Dot))?;
        Ok(Self(index))
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [dot] = tree.collect_nodes(self.0, [NodeKind::Terminal(TerminalKind::Dot)])?;
        Ok(DotView { dot: Dot(dot) })
    }
}
pub struct DotView {
    pub dot: Dot,
}
impl DotView {}
pub struct EndHandle(super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for EndHandle {
    type View = EndView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::End))?;
        Ok(Self(index))
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [r_brace] = tree.collect_nodes(self.0, [NodeKind::Terminal(TerminalKind::RBrace)])?;
        Ok(EndView {
            r_brace: RBrace(r_brace),
        })
    }
}
pub struct EndView {
    pub r_brace: RBrace,
}
impl EndView {}
pub struct ExtHandle(super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for ExtHandle {
    type View = ExtView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Ext))?;
        Ok(Self(index))
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [dollar] = tree.collect_nodes(self.0, [NodeKind::Terminal(TerminalKind::Dollar)])?;
        Ok(ExtView {
            dollar: Dollar(dollar),
        })
    }
}
pub struct ExtView {
    pub dollar: Dollar,
}
impl ExtView {}
pub struct ExtensionNameSpaceHandle(super::tree::CstNodeId);
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
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [ext, ident] = tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::Ext),
                NodeKind::NonTerminal(NonTerminalKind::Ident),
            ],
        )?;
        Ok(ExtensionNameSpaceView {
            ext: ExtHandle(ext),
            ident: IdentHandle(ident),
        })
    }
}
pub struct ExtensionNameSpaceView {
    pub ext: ExtHandle,
    pub ident: IdentHandle,
}
impl ExtensionNameSpaceView {}
pub struct FalseHandle(super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for FalseHandle {
    type View = FalseView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::False))?;
        Ok(Self(index))
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [r#false] = tree.collect_nodes(self.0, [NodeKind::Terminal(TerminalKind::False)])?;
        Ok(FalseView {
            r#false: False(r#false),
        })
    }
}
pub struct FalseView {
    pub r#false: False,
}
impl FalseView {}
pub struct HoleHandle(super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for HoleHandle {
    type View = HoleView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Hole))?;
        Ok(Self(index))
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [hole] = tree.collect_nodes(self.0, [NodeKind::Terminal(TerminalKind::Hole)])?;
        Ok(HoleView { hole: Hole(hole) })
    }
}
pub struct HoleView {
    pub hole: Hole,
}
impl HoleView {}
pub struct IdentHandle(super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for IdentHandle {
    type View = IdentView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Ident))?;
        Ok(Self(index))
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [ident] = tree.collect_nodes(self.0, [NodeKind::Terminal(TerminalKind::Ident)])?;
        Ok(IdentView {
            ident: Ident(ident),
        })
    }
}
pub struct IdentView {
    pub ident: Ident,
}
impl IdentView {}
pub struct InStrHandle(super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for InStrHandle {
    type View = InStrView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::InStr))?;
        Ok(Self(index))
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [in_str] = tree.collect_nodes(self.0, [NodeKind::Terminal(TerminalKind::InStr)])?;
        Ok(InStrView {
            in_str: InStr(in_str),
        })
    }
}
pub struct InStrView {
    pub in_str: InStr,
}
impl InStrView {}
pub struct IntegerHandle(super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for IntegerHandle {
    type View = IntegerView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Integer))?;
        Ok(Self(index))
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [integer] = tree.collect_nodes(self.0, [NodeKind::Terminal(TerminalKind::Integer)])?;
        Ok(IntegerView {
            integer: Integer(integer),
        })
    }
}
pub struct IntegerView {
    pub integer: Integer,
}
impl IntegerView {}
pub struct KeyHandle(super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for KeyHandle {
    type View = KeyView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Key))?;
        Ok(Self(index))
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [key_base, key_opt] = tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::KeyBase),
                NodeKind::NonTerminal(NonTerminalKind::KeyOpt),
            ],
        )?;
        Ok(KeyView {
            key_base: KeyBaseHandle(key_base),
            key_opt: KeyOptHandle(key_opt),
        })
    }
}
pub struct KeyView {
    pub key_base: KeyBaseHandle,
    pub key_opt: KeyOptHandle,
}
impl KeyView {}
pub struct KeyBaseHandle(super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for KeyBaseHandle {
    type View = KeyBaseView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::KeyBase))?;
        Ok(Self(index))
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let mut children = tree.children(self.0);
        let Some(child) = children.next() else {
            return Err(ViewConstructionError::UnexpectedEndOfChildren { parent: self.0 });
        };
        let Some(child_data) = tree.node_data(child) else {
            return Err(ViewConstructionError::NodeIdNotFound { node: child });
        };
        let variant = match child_data.node_kind() {
            NodeKind::NonTerminal(NonTerminalKind::Ident) => KeyBaseView::Ident(IdentHandle(child)),
            NodeKind::NonTerminal(NonTerminalKind::ExtensionNameSpace) => {
                KeyBaseView::ExtensionNameSpace(ExtensionNameSpaceHandle(child))
            }
            NodeKind::NonTerminal(NonTerminalKind::Str) => KeyBaseView::Str(StrHandle(child)),
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
            return Err(ViewConstructionError::UnexpectedExtraNode { node: child });
        }
        Ok(variant)
    }
}
pub enum KeyBaseView {
    Ident(IdentHandle),
    ExtensionNameSpace(ExtensionNameSpaceHandle),
    Str(StrHandle),
    Integer(IntegerHandle),
}
impl KeyBaseView {}
pub struct KeyOptHandle(super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for KeyOptHandle {
    type View = Option<ArrayMarkerHandle>;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::KeyOpt))?;
        Ok(Self(index))
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        if tree.has_no_children(self.0) {
            return Ok(None);
        }
        let [child] = tree.collect_nodes(
            self.0,
            [NodeKind::NonTerminal(NonTerminalKind::ArrayMarker)],
        )?;
        Ok(Some(ArrayMarkerHandle::new(
            child,
            NodeKind::NonTerminal(NonTerminalKind::ArrayMarker),
        )?))
    }
}
pub struct KeysHandle(super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for KeysHandle {
    type View = KeysView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Keys))?;
        Ok(Self(index))
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [key, keys_list] = tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::Key),
                NodeKind::NonTerminal(NonTerminalKind::KeysList),
            ],
        )?;
        Ok(KeysView {
            key: KeyHandle(key),
            keys_list: KeysListHandle(keys_list),
        })
    }
}
pub struct KeysView {
    pub key: KeyHandle,
    pub keys_list: KeysListHandle,
}
impl KeysView {}
pub struct KeysListHandle(super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for KeysListHandle {
    type View = Option<KeysListView>;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(
            index,
            kind,
            NodeKind::NonTerminal(NonTerminalKind::KeysList),
        )?;
        Ok(Self(index))
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        if tree.has_no_children(self.0) {
            return Ok(None);
        }
        let [dot, key, keys_list] = tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::Dot),
                NodeKind::NonTerminal(NonTerminalKind::Key),
                NodeKind::NonTerminal(NonTerminalKind::KeysList),
            ],
        )?;
        Ok(Some(KeysListView {
            dot: DotHandle(dot),
            key: KeyHandle(key),
            keys_list: KeysListHandle(keys_list),
        }))
    }
}
pub struct KeysListView {
    pub dot: DotHandle,
    pub key: KeyHandle,
    pub keys_list: KeysListHandle,
}
impl KeysListView {}
pub struct NamedCodeHandle(super::tree::CstNodeId);
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
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [named_code] =
            tree.collect_nodes(self.0, [NodeKind::Terminal(TerminalKind::NamedCode)])?;
        Ok(NamedCodeView {
            named_code: NamedCode(named_code),
        })
    }
}
pub struct NamedCodeView {
    pub named_code: NamedCode,
}
impl NamedCodeView {}
pub struct NamedCodeBlockHandle(super::tree::CstNodeId);
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
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [named_code_block_begin, code_block_tail_common] = tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::NamedCodeBlockBegin),
                NodeKind::NonTerminal(NonTerminalKind::CodeBlockTailCommon),
            ],
        )?;
        Ok(NamedCodeBlockView {
            named_code_block_begin: NamedCodeBlockBeginHandle(named_code_block_begin),
            code_block_tail_common: CodeBlockTailCommonHandle(code_block_tail_common),
        })
    }
}
pub struct NamedCodeBlockView {
    pub named_code_block_begin: NamedCodeBlockBeginHandle,
    pub code_block_tail_common: CodeBlockTailCommonHandle,
}
impl NamedCodeBlockView {}
pub struct NamedCodeBlockBeginHandle(super::tree::CstNodeId);
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
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [named_code_block_begin] = tree.collect_nodes(
            self.0,
            [NodeKind::Terminal(TerminalKind::NamedCodeBlockBegin)],
        )?;
        Ok(NamedCodeBlockBeginView {
            named_code_block_begin: NamedCodeBlockBegin(named_code_block_begin),
        })
    }
}
pub struct NamedCodeBlockBeginView {
    pub named_code_block_begin: NamedCodeBlockBegin,
}
impl NamedCodeBlockBeginView {}
pub struct NewlineHandle(super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for NewlineHandle {
    type View = NewlineView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Newline))?;
        Ok(Self(index))
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [newline] = tree.collect_nodes(self.0, [NodeKind::Terminal(TerminalKind::Newline)])?;
        Ok(NewlineView {
            newline: Newline(newline),
        })
    }
}
pub struct NewlineView {
    pub newline: Newline,
}
impl NewlineView {}
pub struct NullHandle(super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for NullHandle {
    type View = NullView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Null))?;
        Ok(Self(index))
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [null] = tree.collect_nodes(self.0, [NodeKind::Terminal(TerminalKind::Null)])?;
        Ok(NullView { null: Null(null) })
    }
}
pub struct NullView {
    pub null: Null,
}
impl NullView {}
pub struct ObjectHandle(super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for ObjectHandle {
    type View = ObjectView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Object))?;
        Ok(Self(index))
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [begin, object_list, end] = tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::Begin),
                NodeKind::NonTerminal(NonTerminalKind::ObjectList),
                NodeKind::NonTerminal(NonTerminalKind::End),
            ],
        )?;
        Ok(ObjectView {
            begin: BeginHandle(begin),
            object_list: ObjectListHandle(object_list),
            end: EndHandle(end),
        })
    }
}
pub struct ObjectView {
    pub begin: BeginHandle,
    pub object_list: ObjectListHandle,
    pub end: EndHandle,
}
impl ObjectView {}
pub struct ObjectListHandle(super::tree::CstNodeId);
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
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        if tree.has_no_children(self.0) {
            return Ok(None);
        }
        let [key, bind, value, object_opt, object_list] = tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::Key),
                NodeKind::NonTerminal(NonTerminalKind::Bind),
                NodeKind::NonTerminal(NonTerminalKind::Value),
                NodeKind::NonTerminal(NonTerminalKind::ObjectOpt),
                NodeKind::NonTerminal(NonTerminalKind::ObjectList),
            ],
        )?;
        Ok(Some(ObjectListView {
            key: KeyHandle(key),
            bind: BindHandle(bind),
            value: ValueHandle(value),
            object_opt: ObjectOptHandle(object_opt),
            object_list: ObjectListHandle(object_list),
        }))
    }
}
pub struct ObjectListView {
    pub key: KeyHandle,
    pub bind: BindHandle,
    pub value: ValueHandle,
    pub object_opt: ObjectOptHandle,
    pub object_list: ObjectListHandle,
}
impl ObjectListView {}
pub struct ObjectOptHandle(super::tree::CstNodeId);
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
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        if tree.has_no_children(self.0) {
            return Ok(None);
        }
        let [child] =
            tree.collect_nodes(self.0, [NodeKind::NonTerminal(NonTerminalKind::Comma)])?;
        Ok(Some(CommaHandle::new(
            child,
            NodeKind::NonTerminal(NonTerminalKind::Comma),
        )?))
    }
}
pub struct QuoteHandle(super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for QuoteHandle {
    type View = QuoteView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Quote))?;
        Ok(Self(index))
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [quote] = tree.collect_nodes(self.0, [NodeKind::Terminal(TerminalKind::Quote)])?;
        Ok(QuoteView {
            quote: Quote(quote),
        })
    }
}
pub struct QuoteView {
    pub quote: Quote,
}
impl QuoteView {}
pub struct SectionHandle(super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for SectionHandle {
    type View = SectionView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Section))?;
        Ok(Self(index))
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [at, keys, section_list] = tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::At),
                NodeKind::NonTerminal(NonTerminalKind::Keys),
                NodeKind::NonTerminal(NonTerminalKind::SectionList),
            ],
        )?;
        Ok(SectionView {
            at: AtHandle(at),
            keys: KeysHandle(keys),
            section_list: SectionListHandle(section_list),
        })
    }
}
pub struct SectionView {
    pub at: AtHandle,
    pub keys: KeysHandle,
    pub section_list: SectionListHandle,
}
impl SectionView {}
pub struct SectionBindingHandle(super::tree::CstNodeId);
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
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [begin, swon, end] = tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::Begin),
                NodeKind::NonTerminal(NonTerminalKind::Swon),
                NodeKind::NonTerminal(NonTerminalKind::End),
            ],
        )?;
        Ok(SectionBindingView {
            begin: BeginHandle(begin),
            swon: SwonHandle(swon),
            end: EndHandle(end),
        })
    }
}
pub struct SectionBindingView {
    pub begin: BeginHandle,
    pub swon: SwonHandle,
    pub end: EndHandle,
}
impl SectionBindingView {}
pub struct SectionListHandle(super::tree::CstNodeId);
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
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        if tree.has_no_children(self.0) {
            return Ok(None);
        }
        let [binding, section_list] = tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::Binding),
                NodeKind::NonTerminal(NonTerminalKind::SectionList),
            ],
        )?;
        Ok(Some(SectionListView {
            binding: BindingHandle(binding),
            section_list: SectionListHandle(section_list),
        }))
    }
}
pub struct SectionListView {
    pub binding: BindingHandle,
    pub section_list: SectionListHandle,
}
impl SectionListView {}
pub struct StrHandle(super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for StrHandle {
    type View = StrView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Str))?;
        Ok(Self(index))
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [quote, in_str, quote2] = tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::Quote),
                NodeKind::NonTerminal(NonTerminalKind::InStr),
                NodeKind::NonTerminal(NonTerminalKind::Quote),
            ],
        )?;
        Ok(StrView {
            quote: QuoteHandle(quote),
            in_str: InStrHandle(in_str),
            quote2: QuoteHandle(quote2),
        })
    }
}
pub struct StrView {
    pub quote: QuoteHandle,
    pub in_str: InStrHandle,
    pub quote2: QuoteHandle,
}
impl StrView {}
pub struct StrContinuesHandle(super::tree::CstNodeId);
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
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [str, str_continues_list] = tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::Str),
                NodeKind::NonTerminal(NonTerminalKind::StrContinuesList),
            ],
        )?;
        Ok(StrContinuesView {
            str: StrHandle(str),
            str_continues_list: StrContinuesListHandle(str_continues_list),
        })
    }
}
pub struct StrContinuesView {
    pub str: StrHandle,
    pub str_continues_list: StrContinuesListHandle,
}
impl StrContinuesView {}
pub struct StrContinuesListHandle(super::tree::CstNodeId);
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
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        if tree.has_no_children(self.0) {
            return Ok(None);
        }
        let [r#continue, str, str_continues_list] = tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::Continue),
                NodeKind::NonTerminal(NonTerminalKind::Str),
                NodeKind::NonTerminal(NonTerminalKind::StrContinuesList),
            ],
        )?;
        Ok(Some(StrContinuesListView {
            r#continue: ContinueHandle(r#continue),
            str: StrHandle(str),
            str_continues_list: StrContinuesListHandle(str_continues_list),
        }))
    }
}
pub struct StrContinuesListView {
    pub r#continue: ContinueHandle,
    pub str: StrHandle,
    pub str_continues_list: StrContinuesListHandle,
}
impl StrContinuesListView {}
pub struct SwonHandle(super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for SwonHandle {
    type View = SwonView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Swon))?;
        Ok(Self(index))
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [swon_list, swon_list_0] = tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::SwonList),
                NodeKind::NonTerminal(NonTerminalKind::SwonList0),
            ],
        )?;
        Ok(SwonView {
            swon_list: SwonListHandle(swon_list),
            swon_list_0: SwonList0Handle(swon_list_0),
        })
    }
}
pub struct SwonView {
    pub swon_list: SwonListHandle,
    pub swon_list_0: SwonList0Handle,
}
impl SwonView {}
pub struct SwonListHandle(super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for SwonListHandle {
    type View = Option<SwonListView>;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(
            index,
            kind,
            NodeKind::NonTerminal(NonTerminalKind::SwonList),
        )?;
        Ok(Self(index))
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        if tree.has_no_children(self.0) {
            return Ok(None);
        }
        let [binding, swon_list] = tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::Binding),
                NodeKind::NonTerminal(NonTerminalKind::SwonList),
            ],
        )?;
        Ok(Some(SwonListView {
            binding: BindingHandle(binding),
            swon_list: SwonListHandle(swon_list),
        }))
    }
}
pub struct SwonListView {
    pub binding: BindingHandle,
    pub swon_list: SwonListHandle,
}
impl SwonListView {}
pub struct SwonList0Handle(super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for SwonList0Handle {
    type View = Option<SwonList0View>;
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
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        if tree.has_no_children(self.0) {
            return Ok(None);
        }
        let [section, swon_list_0] = tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::Section),
                NodeKind::NonTerminal(NonTerminalKind::SwonList0),
            ],
        )?;
        Ok(Some(SwonList0View {
            section: SectionHandle(section),
            swon_list_0: SwonList0Handle(swon_list_0),
        }))
    }
}
pub struct SwonList0View {
    pub section: SectionHandle,
    pub swon_list_0: SwonList0Handle,
}
impl SwonList0View {}
pub struct TextHandle(super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for TextHandle {
    type View = TextView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Text))?;
        Ok(Self(index))
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [text] = tree.collect_nodes(self.0, [NodeKind::Terminal(TerminalKind::Text)])?;
        Ok(TextView { text: Text(text) })
    }
}
pub struct TextView {
    pub text: Text,
}
impl TextView {}
pub struct TextBindingHandle(super::tree::CstNodeId);
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
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [text_start, text_binding_opt, text, newline] = tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::TextStart),
                NodeKind::NonTerminal(NonTerminalKind::TextBindingOpt),
                NodeKind::NonTerminal(NonTerminalKind::Text),
                NodeKind::NonTerminal(NonTerminalKind::Newline),
            ],
        )?;
        Ok(TextBindingView {
            text_start: TextStartHandle(text_start),
            text_binding_opt: TextBindingOptHandle(text_binding_opt),
            text: TextHandle(text),
            newline: NewlineHandle(newline),
        })
    }
}
pub struct TextBindingView {
    pub text_start: TextStartHandle,
    pub text_binding_opt: TextBindingOptHandle,
    pub text: TextHandle,
    pub newline: NewlineHandle,
}
impl TextBindingView {}
pub struct TextBindingOptHandle(super::tree::CstNodeId);
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
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        if tree.has_no_children(self.0) {
            return Ok(None);
        }
        let [child] = tree.collect_nodes(self.0, [NodeKind::NonTerminal(NonTerminalKind::Ws)])?;
        Ok(Some(WsHandle::new(
            child,
            NodeKind::NonTerminal(NonTerminalKind::Ws),
        )?))
    }
}
pub struct TextStartHandle(super::tree::CstNodeId);
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
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [text_start] =
            tree.collect_nodes(self.0, [NodeKind::Terminal(TerminalKind::TextStart)])?;
        Ok(TextStartView {
            text_start: TextStart(text_start),
        })
    }
}
pub struct TextStartView {
    pub text_start: TextStart,
}
impl TextStartView {}
pub struct TrueHandle(super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for TrueHandle {
    type View = TrueView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::True))?;
        Ok(Self(index))
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [r#true] = tree.collect_nodes(self.0, [NodeKind::Terminal(TerminalKind::True)])?;
        Ok(TrueView {
            r#true: True(r#true),
        })
    }
}
pub struct TrueView {
    pub r#true: True,
}
impl TrueView {}
pub struct TypedQuoteHandle(super::tree::CstNodeId);
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
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [typed_quote] =
            tree.collect_nodes(self.0, [NodeKind::Terminal(TerminalKind::TypedQuote)])?;
        Ok(TypedQuoteView {
            typed_quote: TypedQuote(typed_quote),
        })
    }
}
pub struct TypedQuoteView {
    pub typed_quote: TypedQuote,
}
impl TypedQuoteView {}
pub struct TypedStrHandle(super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for TypedStrHandle {
    type View = TypedStrView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(
            index,
            kind,
            NodeKind::NonTerminal(NonTerminalKind::TypedStr),
        )?;
        Ok(Self(index))
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [typed_quote, in_str, quote] = tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::TypedQuote),
                NodeKind::NonTerminal(NonTerminalKind::InStr),
                NodeKind::NonTerminal(NonTerminalKind::Quote),
            ],
        )?;
        Ok(TypedStrView {
            typed_quote: TypedQuoteHandle(typed_quote),
            in_str: InStrHandle(in_str),
            quote: QuoteHandle(quote),
        })
    }
}
pub struct TypedStrView {
    pub typed_quote: TypedQuoteHandle,
    pub in_str: InStrHandle,
    pub quote: QuoteHandle,
}
impl TypedStrView {}
pub struct ValueHandle(super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for ValueHandle {
    type View = ValueView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Value))?;
        Ok(Self(index))
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let mut children = tree.children(self.0);
        let Some(child) = children.next() else {
            return Err(ViewConstructionError::UnexpectedEndOfChildren { parent: self.0 });
        };
        let Some(child_data) = tree.node_data(child) else {
            return Err(ViewConstructionError::NodeIdNotFound { node: child });
        };
        let variant = match child_data.node_kind() {
            NodeKind::NonTerminal(NonTerminalKind::Object) => {
                ValueView::Object(ObjectHandle(child))
            }
            NodeKind::NonTerminal(NonTerminalKind::Array) => ValueView::Array(ArrayHandle(child)),
            NodeKind::NonTerminal(NonTerminalKind::Integer) => {
                ValueView::Integer(IntegerHandle(child))
            }
            NodeKind::NonTerminal(NonTerminalKind::Boolean) => {
                ValueView::Boolean(BooleanHandle(child))
            }
            NodeKind::NonTerminal(NonTerminalKind::Null) => ValueView::Null(NullHandle(child)),
            NodeKind::NonTerminal(NonTerminalKind::StrContinues) => {
                ValueView::StrContinues(StrContinuesHandle(child))
            }
            NodeKind::NonTerminal(NonTerminalKind::TypedStr) => {
                ValueView::TypedStr(TypedStrHandle(child))
            }
            NodeKind::NonTerminal(NonTerminalKind::Hole) => ValueView::Hole(HoleHandle(child)),
            NodeKind::NonTerminal(NonTerminalKind::NamedCodeBlock) => {
                ValueView::NamedCodeBlock(NamedCodeBlockHandle(child))
            }
            NodeKind::NonTerminal(NonTerminalKind::CodeBlock) => {
                ValueView::CodeBlock(CodeBlockHandle(child))
            }
            NodeKind::NonTerminal(NonTerminalKind::NamedCode) => {
                ValueView::NamedCode(NamedCodeHandle(child))
            }
            NodeKind::NonTerminal(NonTerminalKind::Code) => ValueView::Code(CodeHandle(child)),
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
            return Err(ViewConstructionError::UnexpectedExtraNode { node: child });
        }
        Ok(variant)
    }
}
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
pub struct ValueBindingHandle(super::tree::CstNodeId);
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
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [bind, value] = tree.collect_nodes(
            self.0,
            [
                NodeKind::NonTerminal(NonTerminalKind::Bind),
                NodeKind::NonTerminal(NonTerminalKind::Value),
            ],
        )?;
        Ok(ValueBindingView {
            bind: BindHandle(bind),
            value: ValueHandle(value),
        })
    }
}
pub struct ValueBindingView {
    pub bind: BindHandle,
    pub value: ValueHandle,
}
impl ValueBindingView {}
pub struct WsHandle(super::tree::CstNodeId);
impl NonTerminalHandle<TerminalKind, NonTerminalKind> for WsHandle {
    type View = WsView;
    fn new(
        index: CstNodeId,
        kind: NodeKind<TerminalKind, NonTerminalKind>,
    ) -> Result<Self, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        assert_node_kind(index, kind, NodeKind::NonTerminal(NonTerminalKind::Ws))?;
        Ok(Self(index))
    }
    fn get_view(
        &self,
        tree: &Cst,
    ) -> Result<Self::View, ViewConstructionError<TerminalKind, NonTerminalKind>> {
        let [ws] = tree.collect_nodes(self.0, [NodeKind::Terminal(TerminalKind::Ws)])?;
        Ok(WsView { ws: Ws(ws) })
    }
}
pub struct WsView {
    pub ws: Ws,
}
impl WsView {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NewLine(pub CstNodeId);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Whitespace(pub CstNodeId);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LineComment(pub CstNodeId);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockComment(pub CstNodeId);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Integer(pub CstNodeId);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct True(pub CstNodeId);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct False(pub CstNodeId);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Null(pub CstNodeId);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Hole(pub CstNodeId);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Quote(pub CstNodeId);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypedQuote(pub CstNodeId);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InStr(pub CstNodeId);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Text(pub CstNodeId);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NamedCode(pub CstNodeId);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Code(pub CstNodeId);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Newline(pub CstNodeId);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ws(pub CstNodeId);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct At(pub CstNodeId);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Dollar(pub CstNodeId);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Dot(pub CstNodeId);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LBrace(pub CstNodeId);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RBrace(pub CstNodeId);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LBracket(pub CstNodeId);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RBracket(pub CstNodeId);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Bind(pub CstNodeId);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Comma(pub CstNodeId);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Esc(pub CstNodeId);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextStart(pub CstNodeId);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ident(pub CstNodeId);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NamedCodeBlockBegin(pub CstNodeId);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CodeBlockDelimiter(pub CstNodeId);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CodeBlockLine(pub CstNodeId);
