use crate::{
    Cst, CstConstructError, NodeKind, CstNode, nodes::*,
    node_kind::{TerminalKind, NonTerminalKind},
    tree::{
        TerminalHandle as _, NonTerminalHandle as _, TerminalData, NonTerminalData,
        CstNodeId, CstFacade,
    },
};
pub trait CstVisitor<F: CstFacade>: CstVisitorSuper<F, Self::Error> {
    type Error;
    fn visit_array(
        &mut self,
        handle: ArrayHandle,
        view: ArrayView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_array_super(handle, view, tree)
    }
    fn visit_array_begin(
        &mut self,
        handle: ArrayBeginHandle,
        view: ArrayBeginView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_array_begin_super(handle, view, tree)
    }
    fn visit_array_end(
        &mut self,
        handle: ArrayEndHandle,
        view: ArrayEndView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_array_end_super(handle, view, tree)
    }
    fn visit_array_list(
        &mut self,
        handle: ArrayListHandle,
        view: ArrayListView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_array_list_super(handle, view, tree)
    }
    fn visit_array_marker(
        &mut self,
        handle: ArrayMarkerHandle,
        view: ArrayMarkerView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_array_marker_super(handle, view, tree)
    }
    fn visit_array_marker_opt(
        &mut self,
        handle: ArrayMarkerOptHandle,
        view: IntegerHandle,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_array_marker_opt_super(handle, view, tree)
    }
    fn visit_array_opt(
        &mut self,
        handle: ArrayOptHandle,
        view: CommaHandle,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_array_opt_super(handle, view, tree)
    }
    fn visit_at(
        &mut self,
        handle: AtHandle,
        view: AtView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_at_super(handle, view, tree)
    }
    fn visit_begin(
        &mut self,
        handle: BeginHandle,
        view: BeginView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_begin_super(handle, view, tree)
    }
    fn visit_bind(
        &mut self,
        handle: BindHandle,
        view: BindView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_bind_super(handle, view, tree)
    }
    fn visit_binding(
        &mut self,
        handle: BindingHandle,
        view: BindingView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_binding_super(handle, view, tree)
    }
    fn visit_binding_rhs(
        &mut self,
        handle: BindingRhsHandle,
        view: BindingRhsView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_binding_rhs_super(handle, view, tree)
    }
    fn visit_boolean(
        &mut self,
        handle: BooleanHandle,
        view: BooleanView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_boolean_super(handle, view, tree)
    }
    fn visit_code(
        &mut self,
        handle: CodeHandle,
        view: CodeView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_code_super(handle, view, tree)
    }
    fn visit_code_block(
        &mut self,
        handle: CodeBlockHandle,
        view: CodeBlockView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_code_block_super(handle, view, tree)
    }
    fn visit_comma(
        &mut self,
        handle: CommaHandle,
        view: CommaView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_comma_super(handle, view, tree)
    }
    fn visit_continue(
        &mut self,
        handle: ContinueHandle,
        view: ContinueView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_continue_super(handle, view, tree)
    }
    fn visit_dot(
        &mut self,
        handle: DotHandle,
        view: DotView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_dot_super(handle, view, tree)
    }
    fn visit_end(
        &mut self,
        handle: EndHandle,
        view: EndView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_end_super(handle, view, tree)
    }
    fn visit_ext(
        &mut self,
        handle: ExtHandle,
        view: ExtView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_ext_super(handle, view, tree)
    }
    fn visit_extension_name_space(
        &mut self,
        handle: ExtensionNameSpaceHandle,
        view: ExtensionNameSpaceView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_extension_name_space_super(handle, view, tree)
    }
    fn visit_false(
        &mut self,
        handle: FalseHandle,
        view: FalseView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_false_super(handle, view, tree)
    }
    fn visit_hole(
        &mut self,
        handle: HoleHandle,
        view: HoleView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_hole_super(handle, view, tree)
    }
    fn visit_ident(
        &mut self,
        handle: IdentHandle,
        view: IdentView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_ident_super(handle, view, tree)
    }
    fn visit_in_str(
        &mut self,
        handle: InStrHandle,
        view: InStrView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_in_str_super(handle, view, tree)
    }
    fn visit_integer(
        &mut self,
        handle: IntegerHandle,
        view: IntegerView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_integer_super(handle, view, tree)
    }
    fn visit_key(
        &mut self,
        handle: KeyHandle,
        view: KeyView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_key_super(handle, view, tree)
    }
    fn visit_key_base(
        &mut self,
        handle: KeyBaseHandle,
        view: KeyBaseView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_key_base_super(handle, view, tree)
    }
    fn visit_key_opt(
        &mut self,
        handle: KeyOptHandle,
        view: ArrayMarkerHandle,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_key_opt_super(handle, view, tree)
    }
    fn visit_keys(
        &mut self,
        handle: KeysHandle,
        view: KeysView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_keys_super(handle, view, tree)
    }
    fn visit_keys_list(
        &mut self,
        handle: KeysListHandle,
        view: KeysListView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_keys_list_super(handle, view, tree)
    }
    fn visit_named_code(
        &mut self,
        handle: NamedCodeHandle,
        view: NamedCodeView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_named_code_super(handle, view, tree)
    }
    fn visit_newline(
        &mut self,
        handle: NewlineHandle,
        view: NewlineView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_newline_super(handle, view, tree)
    }
    fn visit_null(
        &mut self,
        handle: NullHandle,
        view: NullView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_null_super(handle, view, tree)
    }
    fn visit_object(
        &mut self,
        handle: ObjectHandle,
        view: ObjectView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_object_super(handle, view, tree)
    }
    fn visit_object_list(
        &mut self,
        handle: ObjectListHandle,
        view: ObjectListView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_object_list_super(handle, view, tree)
    }
    fn visit_object_opt(
        &mut self,
        handle: ObjectOptHandle,
        view: CommaHandle,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_object_opt_super(handle, view, tree)
    }
    fn visit_quote(
        &mut self,
        handle: QuoteHandle,
        view: QuoteView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_quote_super(handle, view, tree)
    }
    fn visit_section(
        &mut self,
        handle: SectionHandle,
        view: SectionView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_section_super(handle, view, tree)
    }
    fn visit_section_binding(
        &mut self,
        handle: SectionBindingHandle,
        view: SectionBindingView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_section_binding_super(handle, view, tree)
    }
    fn visit_section_body(
        &mut self,
        handle: SectionBodyHandle,
        view: SectionBodyView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_section_body_super(handle, view, tree)
    }
    fn visit_section_body_list(
        &mut self,
        handle: SectionBodyListHandle,
        view: SectionBodyListView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_section_body_list_super(handle, view, tree)
    }
    fn visit_str(
        &mut self,
        handle: StrHandle,
        view: StrView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_str_super(handle, view, tree)
    }
    fn visit_str_continues(
        &mut self,
        handle: StrContinuesHandle,
        view: StrContinuesView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_str_continues_super(handle, view, tree)
    }
    fn visit_str_continues_list(
        &mut self,
        handle: StrContinuesListHandle,
        view: StrContinuesListView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_str_continues_list_super(handle, view, tree)
    }
    fn visit_swon(
        &mut self,
        handle: SwonHandle,
        view: SwonView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_swon_super(handle, view, tree)
    }
    fn visit_swon_bindings(
        &mut self,
        handle: SwonBindingsHandle,
        view: SwonBindingsView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_swon_bindings_super(handle, view, tree)
    }
    fn visit_swon_sections(
        &mut self,
        handle: SwonSectionsHandle,
        view: SwonSectionsView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_swon_sections_super(handle, view, tree)
    }
    fn visit_text(
        &mut self,
        handle: TextHandle,
        view: TextView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_text_super(handle, view, tree)
    }
    fn visit_text_binding(
        &mut self,
        handle: TextBindingHandle,
        view: TextBindingView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_text_binding_super(handle, view, tree)
    }
    fn visit_text_binding_opt(
        &mut self,
        handle: TextBindingOptHandle,
        view: WsHandle,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_text_binding_opt_super(handle, view, tree)
    }
    fn visit_text_start(
        &mut self,
        handle: TextStartHandle,
        view: TextStartView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_text_start_super(handle, view, tree)
    }
    fn visit_true(
        &mut self,
        handle: TrueHandle,
        view: TrueView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_true_super(handle, view, tree)
    }
    fn visit_typed_quote(
        &mut self,
        handle: TypedQuoteHandle,
        view: TypedQuoteView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_typed_quote_super(handle, view, tree)
    }
    fn visit_typed_str(
        &mut self,
        handle: TypedStrHandle,
        view: TypedStrView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_typed_str_super(handle, view, tree)
    }
    fn visit_value(
        &mut self,
        handle: ValueHandle,
        view: ValueView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_value_super(handle, view, tree)
    }
    fn visit_value_binding(
        &mut self,
        handle: ValueBindingHandle,
        view: ValueBindingView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_value_binding_super(handle, view, tree)
    }
    fn visit_ws(
        &mut self,
        handle: WsHandle,
        view: WsView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_ws_super(handle, view, tree)
    }
    fn visit_root(
        &mut self,
        handle: RootHandle,
        view: RootView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_root_super(handle, view, tree)
    }
    fn visit_new_line_terminal(
        &mut self,
        terminal: NewLine,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_new_line_terminal_super(terminal, data, tree)
    }
    fn visit_whitespace_terminal(
        &mut self,
        terminal: Whitespace,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_whitespace_terminal_super(terminal, data, tree)
    }
    fn visit_line_comment_terminal(
        &mut self,
        terminal: LineComment,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_line_comment_terminal_super(terminal, data, tree)
    }
    fn visit_block_comment_terminal(
        &mut self,
        terminal: BlockComment,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_block_comment_terminal_super(terminal, data, tree)
    }
    fn visit_integer_terminal(
        &mut self,
        terminal: Integer,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_integer_terminal_super(terminal, data, tree)
    }
    fn visit_true_terminal(
        &mut self,
        terminal: True,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_true_terminal_super(terminal, data, tree)
    }
    fn visit_false_terminal(
        &mut self,
        terminal: False,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_false_terminal_super(terminal, data, tree)
    }
    fn visit_null_terminal(
        &mut self,
        terminal: Null,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_null_terminal_super(terminal, data, tree)
    }
    fn visit_hole_terminal(
        &mut self,
        terminal: Hole,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_hole_terminal_super(terminal, data, tree)
    }
    fn visit_quote_terminal(
        &mut self,
        terminal: Quote,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_quote_terminal_super(terminal, data, tree)
    }
    fn visit_typed_quote_terminal(
        &mut self,
        terminal: TypedQuote,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_typed_quote_terminal_super(terminal, data, tree)
    }
    fn visit_in_str_terminal(
        &mut self,
        terminal: InStr,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_in_str_terminal_super(terminal, data, tree)
    }
    fn visit_text_terminal(
        &mut self,
        terminal: Text,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_text_terminal_super(terminal, data, tree)
    }
    fn visit_code_block_terminal(
        &mut self,
        terminal: CodeBlock,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_code_block_terminal_super(terminal, data, tree)
    }
    fn visit_named_code_terminal(
        &mut self,
        terminal: NamedCode,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_named_code_terminal_super(terminal, data, tree)
    }
    fn visit_code_terminal(
        &mut self,
        terminal: Code,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_code_terminal_super(terminal, data, tree)
    }
    fn visit_newline_terminal(
        &mut self,
        terminal: Newline,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_newline_terminal_super(terminal, data, tree)
    }
    fn visit_ws_terminal(
        &mut self,
        terminal: Ws,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_ws_terminal_super(terminal, data, tree)
    }
    fn visit_at_terminal(
        &mut self,
        terminal: At,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_at_terminal_super(terminal, data, tree)
    }
    fn visit_dollar_terminal(
        &mut self,
        terminal: Dollar,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_dollar_terminal_super(terminal, data, tree)
    }
    fn visit_dot_terminal(
        &mut self,
        terminal: Dot,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_dot_terminal_super(terminal, data, tree)
    }
    fn visit_l_brace_terminal(
        &mut self,
        terminal: LBrace,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_l_brace_terminal_super(terminal, data, tree)
    }
    fn visit_r_brace_terminal(
        &mut self,
        terminal: RBrace,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_r_brace_terminal_super(terminal, data, tree)
    }
    fn visit_l_bracket_terminal(
        &mut self,
        terminal: LBracket,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_l_bracket_terminal_super(terminal, data, tree)
    }
    fn visit_r_bracket_terminal(
        &mut self,
        terminal: RBracket,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_r_bracket_terminal_super(terminal, data, tree)
    }
    fn visit_bind_terminal(
        &mut self,
        terminal: Bind,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_bind_terminal_super(terminal, data, tree)
    }
    fn visit_comma_terminal(
        &mut self,
        terminal: Comma,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_comma_terminal_super(terminal, data, tree)
    }
    fn visit_esc_terminal(
        &mut self,
        terminal: Esc,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_esc_terminal_super(terminal, data, tree)
    }
    fn visit_text_start_terminal(
        &mut self,
        terminal: TextStart,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_text_start_terminal_super(terminal, data, tree)
    }
    fn visit_ident_terminal(
        &mut self,
        terminal: Ident,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_ident_terminal_super(terminal, data, tree)
    }
    fn visit_non_terminal(
        &mut self,
        id: CstNodeId,
        kind: NonTerminalKind,
        data: NonTerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_non_terminal_super(id, kind, data, tree)
    }
    fn visit_non_terminal_close(
        &mut self,
        id: CstNodeId,
        kind: NonTerminalKind,
        data: NonTerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_non_terminal_close_super(id, kind, data, tree)
    }
    fn visit_terminal(
        &mut self,
        id: CstNodeId,
        kind: TerminalKind,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_terminal_super(id, kind, data, tree)
    }
    /// This method is called when a construct view fails.
    /// If you return Ok(()), the error is not propagated.
    fn then_construct_error(
        &mut self,
        node_data: Option<CstNode>,
        parent: CstNodeId,
        kind: NodeKind,
        error: CstConstructError,
        tree: &F,
    ) -> Result<(), Self::Error> {
        let _error = error;
        self.recover_error(node_data, parent, kind, tree)
    }
}
mod private {
    pub trait Sealed<F> {}
}
pub trait CstVisitorSuper<F: CstFacade, E>: private::Sealed<F> {
    fn visit_array_handle(&mut self, handle: ArrayHandle, tree: &F) -> Result<(), E>;
    fn visit_array_super(
        &mut self,
        handle: ArrayHandle,
        view: ArrayView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_array_begin_handle(
        &mut self,
        handle: ArrayBeginHandle,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_array_begin_super(
        &mut self,
        handle: ArrayBeginHandle,
        view: ArrayBeginView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_array_end_handle(
        &mut self,
        handle: ArrayEndHandle,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_array_end_super(
        &mut self,
        handle: ArrayEndHandle,
        view: ArrayEndView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_array_list_handle(
        &mut self,
        handle: ArrayListHandle,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_array_list_super(
        &mut self,
        handle: ArrayListHandle,
        view: ArrayListView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_array_marker_handle(
        &mut self,
        handle: ArrayMarkerHandle,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_array_marker_super(
        &mut self,
        handle: ArrayMarkerHandle,
        view: ArrayMarkerView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_array_marker_opt_handle(
        &mut self,
        handle: ArrayMarkerOptHandle,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_array_marker_opt_super(
        &mut self,
        handle: ArrayMarkerOptHandle,
        view: IntegerHandle,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_array_opt_handle(
        &mut self,
        handle: ArrayOptHandle,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_array_opt_super(
        &mut self,
        handle: ArrayOptHandle,
        view: CommaHandle,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_at_handle(&mut self, handle: AtHandle, tree: &F) -> Result<(), E>;
    fn visit_at_super(
        &mut self,
        handle: AtHandle,
        view: AtView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_begin_handle(&mut self, handle: BeginHandle, tree: &F) -> Result<(), E>;
    fn visit_begin_super(
        &mut self,
        handle: BeginHandle,
        view: BeginView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_bind_handle(&mut self, handle: BindHandle, tree: &F) -> Result<(), E>;
    fn visit_bind_super(
        &mut self,
        handle: BindHandle,
        view: BindView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_binding_handle(&mut self, handle: BindingHandle, tree: &F) -> Result<(), E>;
    fn visit_binding_super(
        &mut self,
        handle: BindingHandle,
        view: BindingView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_binding_rhs_handle(
        &mut self,
        handle: BindingRhsHandle,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_binding_rhs_super(
        &mut self,
        handle: BindingRhsHandle,
        view: BindingRhsView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_boolean_handle(&mut self, handle: BooleanHandle, tree: &F) -> Result<(), E>;
    fn visit_boolean_super(
        &mut self,
        handle: BooleanHandle,
        view: BooleanView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_code_handle(&mut self, handle: CodeHandle, tree: &F) -> Result<(), E>;
    fn visit_code_super(
        &mut self,
        handle: CodeHandle,
        view: CodeView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_code_block_handle(
        &mut self,
        handle: CodeBlockHandle,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_code_block_super(
        &mut self,
        handle: CodeBlockHandle,
        view: CodeBlockView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_comma_handle(&mut self, handle: CommaHandle, tree: &F) -> Result<(), E>;
    fn visit_comma_super(
        &mut self,
        handle: CommaHandle,
        view: CommaView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_continue_handle(
        &mut self,
        handle: ContinueHandle,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_continue_super(
        &mut self,
        handle: ContinueHandle,
        view: ContinueView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_dot_handle(&mut self, handle: DotHandle, tree: &F) -> Result<(), E>;
    fn visit_dot_super(
        &mut self,
        handle: DotHandle,
        view: DotView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_end_handle(&mut self, handle: EndHandle, tree: &F) -> Result<(), E>;
    fn visit_end_super(
        &mut self,
        handle: EndHandle,
        view: EndView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_ext_handle(&mut self, handle: ExtHandle, tree: &F) -> Result<(), E>;
    fn visit_ext_super(
        &mut self,
        handle: ExtHandle,
        view: ExtView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_extension_name_space_handle(
        &mut self,
        handle: ExtensionNameSpaceHandle,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_extension_name_space_super(
        &mut self,
        handle: ExtensionNameSpaceHandle,
        view: ExtensionNameSpaceView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_false_handle(&mut self, handle: FalseHandle, tree: &F) -> Result<(), E>;
    fn visit_false_super(
        &mut self,
        handle: FalseHandle,
        view: FalseView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_hole_handle(&mut self, handle: HoleHandle, tree: &F) -> Result<(), E>;
    fn visit_hole_super(
        &mut self,
        handle: HoleHandle,
        view: HoleView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_ident_handle(&mut self, handle: IdentHandle, tree: &F) -> Result<(), E>;
    fn visit_ident_super(
        &mut self,
        handle: IdentHandle,
        view: IdentView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_in_str_handle(&mut self, handle: InStrHandle, tree: &F) -> Result<(), E>;
    fn visit_in_str_super(
        &mut self,
        handle: InStrHandle,
        view: InStrView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_integer_handle(&mut self, handle: IntegerHandle, tree: &F) -> Result<(), E>;
    fn visit_integer_super(
        &mut self,
        handle: IntegerHandle,
        view: IntegerView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_key_handle(&mut self, handle: KeyHandle, tree: &F) -> Result<(), E>;
    fn visit_key_super(
        &mut self,
        handle: KeyHandle,
        view: KeyView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_key_base_handle(
        &mut self,
        handle: KeyBaseHandle,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_key_base_super(
        &mut self,
        handle: KeyBaseHandle,
        view: KeyBaseView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_key_opt_handle(&mut self, handle: KeyOptHandle, tree: &F) -> Result<(), E>;
    fn visit_key_opt_super(
        &mut self,
        handle: KeyOptHandle,
        view: ArrayMarkerHandle,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_keys_handle(&mut self, handle: KeysHandle, tree: &F) -> Result<(), E>;
    fn visit_keys_super(
        &mut self,
        handle: KeysHandle,
        view: KeysView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_keys_list_handle(
        &mut self,
        handle: KeysListHandle,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_keys_list_super(
        &mut self,
        handle: KeysListHandle,
        view: KeysListView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_named_code_handle(
        &mut self,
        handle: NamedCodeHandle,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_named_code_super(
        &mut self,
        handle: NamedCodeHandle,
        view: NamedCodeView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_newline_handle(&mut self, handle: NewlineHandle, tree: &F) -> Result<(), E>;
    fn visit_newline_super(
        &mut self,
        handle: NewlineHandle,
        view: NewlineView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_null_handle(&mut self, handle: NullHandle, tree: &F) -> Result<(), E>;
    fn visit_null_super(
        &mut self,
        handle: NullHandle,
        view: NullView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_object_handle(&mut self, handle: ObjectHandle, tree: &F) -> Result<(), E>;
    fn visit_object_super(
        &mut self,
        handle: ObjectHandle,
        view: ObjectView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_object_list_handle(
        &mut self,
        handle: ObjectListHandle,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_object_list_super(
        &mut self,
        handle: ObjectListHandle,
        view: ObjectListView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_object_opt_handle(
        &mut self,
        handle: ObjectOptHandle,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_object_opt_super(
        &mut self,
        handle: ObjectOptHandle,
        view: CommaHandle,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_quote_handle(&mut self, handle: QuoteHandle, tree: &F) -> Result<(), E>;
    fn visit_quote_super(
        &mut self,
        handle: QuoteHandle,
        view: QuoteView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_section_handle(&mut self, handle: SectionHandle, tree: &F) -> Result<(), E>;
    fn visit_section_super(
        &mut self,
        handle: SectionHandle,
        view: SectionView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_section_binding_handle(
        &mut self,
        handle: SectionBindingHandle,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_section_binding_super(
        &mut self,
        handle: SectionBindingHandle,
        view: SectionBindingView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_section_body_handle(
        &mut self,
        handle: SectionBodyHandle,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_section_body_super(
        &mut self,
        handle: SectionBodyHandle,
        view: SectionBodyView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_section_body_list_handle(
        &mut self,
        handle: SectionBodyListHandle,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_section_body_list_super(
        &mut self,
        handle: SectionBodyListHandle,
        view: SectionBodyListView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_str_handle(&mut self, handle: StrHandle, tree: &F) -> Result<(), E>;
    fn visit_str_super(
        &mut self,
        handle: StrHandle,
        view: StrView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_str_continues_handle(
        &mut self,
        handle: StrContinuesHandle,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_str_continues_super(
        &mut self,
        handle: StrContinuesHandle,
        view: StrContinuesView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_str_continues_list_handle(
        &mut self,
        handle: StrContinuesListHandle,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_str_continues_list_super(
        &mut self,
        handle: StrContinuesListHandle,
        view: StrContinuesListView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_swon_handle(&mut self, handle: SwonHandle, tree: &F) -> Result<(), E>;
    fn visit_swon_super(
        &mut self,
        handle: SwonHandle,
        view: SwonView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_swon_bindings_handle(
        &mut self,
        handle: SwonBindingsHandle,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_swon_bindings_super(
        &mut self,
        handle: SwonBindingsHandle,
        view: SwonBindingsView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_swon_sections_handle(
        &mut self,
        handle: SwonSectionsHandle,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_swon_sections_super(
        &mut self,
        handle: SwonSectionsHandle,
        view: SwonSectionsView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_text_handle(&mut self, handle: TextHandle, tree: &F) -> Result<(), E>;
    fn visit_text_super(
        &mut self,
        handle: TextHandle,
        view: TextView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_text_binding_handle(
        &mut self,
        handle: TextBindingHandle,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_text_binding_super(
        &mut self,
        handle: TextBindingHandle,
        view: TextBindingView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_text_binding_opt_handle(
        &mut self,
        handle: TextBindingOptHandle,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_text_binding_opt_super(
        &mut self,
        handle: TextBindingOptHandle,
        view: WsHandle,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_text_start_handle(
        &mut self,
        handle: TextStartHandle,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_text_start_super(
        &mut self,
        handle: TextStartHandle,
        view: TextStartView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_true_handle(&mut self, handle: TrueHandle, tree: &F) -> Result<(), E>;
    fn visit_true_super(
        &mut self,
        handle: TrueHandle,
        view: TrueView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_typed_quote_handle(
        &mut self,
        handle: TypedQuoteHandle,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_typed_quote_super(
        &mut self,
        handle: TypedQuoteHandle,
        view: TypedQuoteView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_typed_str_handle(
        &mut self,
        handle: TypedStrHandle,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_typed_str_super(
        &mut self,
        handle: TypedStrHandle,
        view: TypedStrView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_value_handle(&mut self, handle: ValueHandle, tree: &F) -> Result<(), E>;
    fn visit_value_super(
        &mut self,
        handle: ValueHandle,
        view: ValueView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_value_binding_handle(
        &mut self,
        handle: ValueBindingHandle,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_value_binding_super(
        &mut self,
        handle: ValueBindingHandle,
        view: ValueBindingView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_ws_handle(&mut self, handle: WsHandle, tree: &F) -> Result<(), E>;
    fn visit_ws_super(
        &mut self,
        handle: WsHandle,
        view: WsView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_root_handle(&mut self, handle: RootHandle, tree: &F) -> Result<(), E>;
    fn visit_root_super(
        &mut self,
        handle: RootHandle,
        view: RootView,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_new_line_terminal_super(
        &mut self,
        terminal: NewLine,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_whitespace_terminal_super(
        &mut self,
        terminal: Whitespace,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_line_comment_terminal_super(
        &mut self,
        terminal: LineComment,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_block_comment_terminal_super(
        &mut self,
        terminal: BlockComment,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_integer_terminal_super(
        &mut self,
        terminal: Integer,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_true_terminal_super(
        &mut self,
        terminal: True,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_false_terminal_super(
        &mut self,
        terminal: False,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_null_terminal_super(
        &mut self,
        terminal: Null,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_hole_terminal_super(
        &mut self,
        terminal: Hole,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_quote_terminal_super(
        &mut self,
        terminal: Quote,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_typed_quote_terminal_super(
        &mut self,
        terminal: TypedQuote,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_in_str_terminal_super(
        &mut self,
        terminal: InStr,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_text_terminal_super(
        &mut self,
        terminal: Text,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_code_block_terminal_super(
        &mut self,
        terminal: CodeBlock,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_named_code_terminal_super(
        &mut self,
        terminal: NamedCode,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_code_terminal_super(
        &mut self,
        terminal: Code,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_newline_terminal_super(
        &mut self,
        terminal: Newline,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_ws_terminal_super(
        &mut self,
        terminal: Ws,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_at_terminal_super(
        &mut self,
        terminal: At,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_dollar_terminal_super(
        &mut self,
        terminal: Dollar,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_dot_terminal_super(
        &mut self,
        terminal: Dot,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_l_brace_terminal_super(
        &mut self,
        terminal: LBrace,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_r_brace_terminal_super(
        &mut self,
        terminal: RBrace,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_l_bracket_terminal_super(
        &mut self,
        terminal: LBracket,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_r_bracket_terminal_super(
        &mut self,
        terminal: RBracket,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_bind_terminal_super(
        &mut self,
        terminal: Bind,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_comma_terminal_super(
        &mut self,
        terminal: Comma,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_esc_terminal_super(
        &mut self,
        terminal: Esc,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_text_start_terminal_super(
        &mut self,
        terminal: TextStart,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_ident_terminal_super(
        &mut self,
        terminal: Ident,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_non_terminal_super(
        &mut self,
        id: CstNodeId,
        kind: NonTerminalKind,
        data: NonTerminalData,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_non_terminal_close_super(
        &mut self,
        id: CstNodeId,
        kind: NonTerminalKind,
        data: NonTerminalData,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_terminal_super(
        &mut self,
        id: CstNodeId,
        kind: TerminalKind,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_any(&mut self, id: CstNodeId, node: CstNode, tree: &F) -> Result<(), E>;
    /// Recover from a construct error. This eagerly visits the children of the node.
    fn recover_error(
        &mut self,
        node_data: Option<CstNode>,
        id: CstNodeId,
        kind: NodeKind,
        tree: &F,
    ) -> Result<(), E>;
}
impl<V: CstVisitor<F>, F: CstFacade> private::Sealed<F> for V {}
impl<V: CstVisitor<F>, F: CstFacade> CstVisitorSuper<F, V::Error> for V {
    fn visit_array_handle(
        &mut self,
        handle: ArrayHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (visit.visit_array(handle, view, tree), visit),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_array_begin_handle(
        &mut self,
        handle: ArrayBeginHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (
                    visit.visit_array_begin(handle, view, tree),
                    visit,
                ),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_array_end_handle(
        &mut self,
        handle: ArrayEndHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (
                    visit.visit_array_end(handle, view, tree),
                    visit,
                ),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_array_list_handle(
        &mut self,
        handle: ArrayListHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (
                    if let Some(view) = view {
                        visit.visit_array_list(handle, view, tree)
                    } else {
                        Ok(())
                    },
                    visit,
                ),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_array_marker_handle(
        &mut self,
        handle: ArrayMarkerHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (
                    visit.visit_array_marker(handle, view, tree),
                    visit,
                ),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_array_marker_opt_handle(
        &mut self,
        handle: ArrayMarkerOptHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (
                    if let Some(view) = view {
                        visit.visit_array_marker_opt(handle, view, tree)
                    } else {
                        Ok(())
                    },
                    visit,
                ),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_array_opt_handle(
        &mut self,
        handle: ArrayOptHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (
                    if let Some(view) = view {
                        visit.visit_array_opt(handle, view, tree)
                    } else {
                        Ok(())
                    },
                    visit,
                ),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_at_handle(&mut self, handle: AtHandle, tree: &F) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (visit.visit_at(handle, view, tree), visit),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_begin_handle(
        &mut self,
        handle: BeginHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (visit.visit_begin(handle, view, tree), visit),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_bind_handle(
        &mut self,
        handle: BindHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (visit.visit_bind(handle, view, tree), visit),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_binding_handle(
        &mut self,
        handle: BindingHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (
                    visit.visit_binding(handle, view, tree),
                    visit,
                ),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_binding_rhs_handle(
        &mut self,
        handle: BindingRhsHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (
                    visit.visit_binding_rhs(handle, view, tree),
                    visit,
                ),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_boolean_handle(
        &mut self,
        handle: BooleanHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (
                    visit.visit_boolean(handle, view, tree),
                    visit,
                ),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_code_handle(
        &mut self,
        handle: CodeHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (visit.visit_code(handle, view, tree), visit),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_code_block_handle(
        &mut self,
        handle: CodeBlockHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (
                    visit.visit_code_block(handle, view, tree),
                    visit,
                ),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_comma_handle(
        &mut self,
        handle: CommaHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (visit.visit_comma(handle, view, tree), visit),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_continue_handle(
        &mut self,
        handle: ContinueHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (
                    visit.visit_continue(handle, view, tree),
                    visit,
                ),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_dot_handle(&mut self, handle: DotHandle, tree: &F) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (visit.visit_dot(handle, view, tree), visit),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_end_handle(&mut self, handle: EndHandle, tree: &F) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (visit.visit_end(handle, view, tree), visit),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_ext_handle(&mut self, handle: ExtHandle, tree: &F) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (visit.visit_ext(handle, view, tree), visit),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_extension_name_space_handle(
        &mut self,
        handle: ExtensionNameSpaceHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (
                    visit.visit_extension_name_space(handle, view, tree),
                    visit,
                ),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_false_handle(
        &mut self,
        handle: FalseHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (visit.visit_false(handle, view, tree), visit),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_hole_handle(
        &mut self,
        handle: HoleHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (visit.visit_hole(handle, view, tree), visit),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_ident_handle(
        &mut self,
        handle: IdentHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (visit.visit_ident(handle, view, tree), visit),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_in_str_handle(
        &mut self,
        handle: InStrHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (visit.visit_in_str(handle, view, tree), visit),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_integer_handle(
        &mut self,
        handle: IntegerHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (
                    visit.visit_integer(handle, view, tree),
                    visit,
                ),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_key_handle(&mut self, handle: KeyHandle, tree: &F) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (visit.visit_key(handle, view, tree), visit),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_key_base_handle(
        &mut self,
        handle: KeyBaseHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (
                    visit.visit_key_base(handle, view, tree),
                    visit,
                ),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_key_opt_handle(
        &mut self,
        handle: KeyOptHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (
                    if let Some(view) = view {
                        visit.visit_key_opt(handle, view, tree)
                    } else {
                        Ok(())
                    },
                    visit,
                ),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_keys_handle(
        &mut self,
        handle: KeysHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (visit.visit_keys(handle, view, tree), visit),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_keys_list_handle(
        &mut self,
        handle: KeysListHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (
                    if let Some(view) = view {
                        visit.visit_keys_list(handle, view, tree)
                    } else {
                        Ok(())
                    },
                    visit,
                ),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_named_code_handle(
        &mut self,
        handle: NamedCodeHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (
                    visit.visit_named_code(handle, view, tree),
                    visit,
                ),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_newline_handle(
        &mut self,
        handle: NewlineHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (
                    visit.visit_newline(handle, view, tree),
                    visit,
                ),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_null_handle(
        &mut self,
        handle: NullHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (visit.visit_null(handle, view, tree), visit),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_object_handle(
        &mut self,
        handle: ObjectHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (visit.visit_object(handle, view, tree), visit),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_object_list_handle(
        &mut self,
        handle: ObjectListHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (
                    if let Some(view) = view {
                        visit.visit_object_list(handle, view, tree)
                    } else {
                        Ok(())
                    },
                    visit,
                ),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_object_opt_handle(
        &mut self,
        handle: ObjectOptHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (
                    if let Some(view) = view {
                        visit.visit_object_opt(handle, view, tree)
                    } else {
                        Ok(())
                    },
                    visit,
                ),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_quote_handle(
        &mut self,
        handle: QuoteHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (visit.visit_quote(handle, view, tree), visit),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_section_handle(
        &mut self,
        handle: SectionHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (
                    visit.visit_section(handle, view, tree),
                    visit,
                ),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_section_binding_handle(
        &mut self,
        handle: SectionBindingHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (
                    visit.visit_section_binding(handle, view, tree),
                    visit,
                ),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_section_body_handle(
        &mut self,
        handle: SectionBodyHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (
                    visit.visit_section_body(handle, view, tree),
                    visit,
                ),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_section_body_list_handle(
        &mut self,
        handle: SectionBodyListHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (
                    if let Some(view) = view {
                        visit.visit_section_body_list(handle, view, tree)
                    } else {
                        Ok(())
                    },
                    visit,
                ),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_str_handle(&mut self, handle: StrHandle, tree: &F) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (visit.visit_str(handle, view, tree), visit),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_str_continues_handle(
        &mut self,
        handle: StrContinuesHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (
                    visit.visit_str_continues(handle, view, tree),
                    visit,
                ),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_str_continues_list_handle(
        &mut self,
        handle: StrContinuesListHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (
                    if let Some(view) = view {
                        visit.visit_str_continues_list(handle, view, tree)
                    } else {
                        Ok(())
                    },
                    visit,
                ),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_swon_handle(
        &mut self,
        handle: SwonHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (visit.visit_swon(handle, view, tree), visit),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_swon_bindings_handle(
        &mut self,
        handle: SwonBindingsHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (
                    if let Some(view) = view {
                        visit.visit_swon_bindings(handle, view, tree)
                    } else {
                        Ok(())
                    },
                    visit,
                ),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_swon_sections_handle(
        &mut self,
        handle: SwonSectionsHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (
                    if let Some(view) = view {
                        visit.visit_swon_sections(handle, view, tree)
                    } else {
                        Ok(())
                    },
                    visit,
                ),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_text_handle(
        &mut self,
        handle: TextHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (visit.visit_text(handle, view, tree), visit),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_text_binding_handle(
        &mut self,
        handle: TextBindingHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (
                    visit.visit_text_binding(handle, view, tree),
                    visit,
                ),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_text_binding_opt_handle(
        &mut self,
        handle: TextBindingOptHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (
                    if let Some(view) = view {
                        visit.visit_text_binding_opt(handle, view, tree)
                    } else {
                        Ok(())
                    },
                    visit,
                ),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_text_start_handle(
        &mut self,
        handle: TextStartHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (
                    visit.visit_text_start(handle, view, tree),
                    visit,
                ),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_true_handle(
        &mut self,
        handle: TrueHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (visit.visit_true(handle, view, tree), visit),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_typed_quote_handle(
        &mut self,
        handle: TypedQuoteHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (
                    visit.visit_typed_quote(handle, view, tree),
                    visit,
                ),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_typed_str_handle(
        &mut self,
        handle: TypedStrHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (
                    visit.visit_typed_str(handle, view, tree),
                    visit,
                ),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_value_handle(
        &mut self,
        handle: ValueHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (visit.visit_value(handle, view, tree), visit),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_value_binding_handle(
        &mut self,
        handle: ValueBindingHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (
                    visit.visit_value_binding(handle, view, tree),
                    visit,
                ),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_ws_handle(&mut self, handle: WsHandle, tree: &F) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (visit.visit_ws(handle, view, tree), visit),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_root_handle(
        &mut self,
        handle: RootHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let nt_data = match tree.get_non_terminal(handle.node_id(), handle.kind()) {
            Ok(nt_data) => nt_data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), handle.kind(), nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (visit.visit_root(handle, view, tree), visit),
                self,
            )
            .map_err(|e| e.extract_error())
        {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(Ok(e)) => Err(e),
            Err(Err(e)) => {
                self.then_construct_error(
                    Some(CstNode::new_non_terminal(handle.kind(), nt_data)),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), handle.kind(), nt_data, tree)?;
        result
    }
    fn visit_array_super(
        &mut self,
        handle: ArrayHandle,
        view_param: ArrayView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let ArrayView { array_begin, array_list, array_end } = view_param;
        self.visit_array_begin_handle(array_begin, tree)?;
        self.visit_array_list_handle(array_list, tree)?;
        self.visit_array_end_handle(array_end, tree)?;
        Ok(())
    }
    fn visit_array_begin_super(
        &mut self,
        handle: ArrayBeginHandle,
        view_param: ArrayBeginView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let ArrayBeginView { l_bracket } = view_param;
        let data = match l_bracket.get_data(tree) {
            Ok(data) => data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        l_bracket.0,
                        NodeKind::Terminal(l_bracket.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_l_bracket_terminal(l_bracket, data, tree)?;
        Ok(())
    }
    fn visit_array_end_super(
        &mut self,
        handle: ArrayEndHandle,
        view_param: ArrayEndView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let ArrayEndView { r_bracket } = view_param;
        let data = match r_bracket.get_data(tree) {
            Ok(data) => data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        r_bracket.0,
                        NodeKind::Terminal(r_bracket.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_r_bracket_terminal(r_bracket, data, tree)?;
        Ok(())
    }
    fn visit_array_list_super(
        &mut self,
        handle: ArrayListHandle,
        view_param: ArrayListView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let ArrayListView { value, array_opt, array_list } = view_param;
        self.visit_value_handle(value, tree)?;
        self.visit_array_opt_handle(array_opt, tree)?;
        self.visit_array_list_handle(array_list, tree)?;
        Ok(())
    }
    fn visit_array_marker_super(
        &mut self,
        handle: ArrayMarkerHandle,
        view_param: ArrayMarkerView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let ArrayMarkerView { array_begin, array_marker_opt, array_end } = view_param;
        self.visit_array_begin_handle(array_begin, tree)?;
        self.visit_array_marker_opt_handle(array_marker_opt, tree)?;
        self.visit_array_end_handle(array_end, tree)?;
        Ok(())
    }
    fn visit_array_marker_opt_super(
        &mut self,
        handle: ArrayMarkerOptHandle,
        view_param: IntegerHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        self.visit_integer_handle(view_param, tree)?;
        Ok(())
    }
    fn visit_array_opt_super(
        &mut self,
        handle: ArrayOptHandle,
        view_param: CommaHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        self.visit_comma_handle(view_param, tree)?;
        Ok(())
    }
    fn visit_at_super(
        &mut self,
        handle: AtHandle,
        view_param: AtView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let AtView { at } = view_param;
        let data = match at.get_data(tree) {
            Ok(data) => data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        at.0,
                        NodeKind::Terminal(at.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_at_terminal(at, data, tree)?;
        Ok(())
    }
    fn visit_begin_super(
        &mut self,
        handle: BeginHandle,
        view_param: BeginView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let BeginView { l_brace } = view_param;
        let data = match l_brace.get_data(tree) {
            Ok(data) => data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        l_brace.0,
                        NodeKind::Terminal(l_brace.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_l_brace_terminal(l_brace, data, tree)?;
        Ok(())
    }
    fn visit_bind_super(
        &mut self,
        handle: BindHandle,
        view_param: BindView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let BindView { bind } = view_param;
        let data = match bind.get_data(tree) {
            Ok(data) => data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        bind.0,
                        NodeKind::Terminal(bind.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_bind_terminal(bind, data, tree)?;
        Ok(())
    }
    fn visit_binding_super(
        &mut self,
        handle: BindingHandle,
        view_param: BindingView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let BindingView { keys, binding_rhs } = view_param;
        self.visit_keys_handle(keys, tree)?;
        self.visit_binding_rhs_handle(binding_rhs, tree)?;
        Ok(())
    }
    fn visit_binding_rhs_super(
        &mut self,
        handle: BindingRhsHandle,
        view_param: BindingRhsView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        match view_param {
            BindingRhsView::ValueBinding(item) => {
                self.visit_value_binding_handle(item, tree)?;
            }
            BindingRhsView::SectionBinding(item) => {
                self.visit_section_binding_handle(item, tree)?;
            }
            BindingRhsView::TextBinding(item) => {
                self.visit_text_binding_handle(item, tree)?;
            }
        }
        Ok(())
    }
    fn visit_boolean_super(
        &mut self,
        handle: BooleanHandle,
        view_param: BooleanView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        match view_param {
            BooleanView::True(item) => {
                self.visit_true_handle(item, tree)?;
            }
            BooleanView::False(item) => {
                self.visit_false_handle(item, tree)?;
            }
        }
        Ok(())
    }
    fn visit_code_super(
        &mut self,
        handle: CodeHandle,
        view_param: CodeView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let CodeView { code } = view_param;
        let data = match code.get_data(tree) {
            Ok(data) => data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        code.0,
                        NodeKind::Terminal(code.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_code_terminal(code, data, tree)?;
        Ok(())
    }
    fn visit_code_block_super(
        &mut self,
        handle: CodeBlockHandle,
        view_param: CodeBlockView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let CodeBlockView { code_block } = view_param;
        let data = match code_block.get_data(tree) {
            Ok(data) => data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        code_block.0,
                        NodeKind::Terminal(code_block.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_code_block_terminal(code_block, data, tree)?;
        Ok(())
    }
    fn visit_comma_super(
        &mut self,
        handle: CommaHandle,
        view_param: CommaView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let CommaView { comma } = view_param;
        let data = match comma.get_data(tree) {
            Ok(data) => data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        comma.0,
                        NodeKind::Terminal(comma.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_comma_terminal(comma, data, tree)?;
        Ok(())
    }
    fn visit_continue_super(
        &mut self,
        handle: ContinueHandle,
        view_param: ContinueView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let ContinueView { esc } = view_param;
        let data = match esc.get_data(tree) {
            Ok(data) => data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        esc.0,
                        NodeKind::Terminal(esc.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_esc_terminal(esc, data, tree)?;
        Ok(())
    }
    fn visit_dot_super(
        &mut self,
        handle: DotHandle,
        view_param: DotView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let DotView { dot } = view_param;
        let data = match dot.get_data(tree) {
            Ok(data) => data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        dot.0,
                        NodeKind::Terminal(dot.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_dot_terminal(dot, data, tree)?;
        Ok(())
    }
    fn visit_end_super(
        &mut self,
        handle: EndHandle,
        view_param: EndView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let EndView { r_brace } = view_param;
        let data = match r_brace.get_data(tree) {
            Ok(data) => data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        r_brace.0,
                        NodeKind::Terminal(r_brace.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_r_brace_terminal(r_brace, data, tree)?;
        Ok(())
    }
    fn visit_ext_super(
        &mut self,
        handle: ExtHandle,
        view_param: ExtView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let ExtView { dollar } = view_param;
        let data = match dollar.get_data(tree) {
            Ok(data) => data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        dollar.0,
                        NodeKind::Terminal(dollar.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_dollar_terminal(dollar, data, tree)?;
        Ok(())
    }
    fn visit_extension_name_space_super(
        &mut self,
        handle: ExtensionNameSpaceHandle,
        view_param: ExtensionNameSpaceView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let ExtensionNameSpaceView { ext, ident } = view_param;
        self.visit_ext_handle(ext, tree)?;
        self.visit_ident_handle(ident, tree)?;
        Ok(())
    }
    fn visit_false_super(
        &mut self,
        handle: FalseHandle,
        view_param: FalseView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let FalseView { r#false } = view_param;
        let data = match r#false.get_data(tree) {
            Ok(data) => data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        r#false.0,
                        NodeKind::Terminal(r#false.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_false_terminal(r#false, data, tree)?;
        Ok(())
    }
    fn visit_hole_super(
        &mut self,
        handle: HoleHandle,
        view_param: HoleView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let HoleView { hole } = view_param;
        let data = match hole.get_data(tree) {
            Ok(data) => data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        hole.0,
                        NodeKind::Terminal(hole.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_hole_terminal(hole, data, tree)?;
        Ok(())
    }
    fn visit_ident_super(
        &mut self,
        handle: IdentHandle,
        view_param: IdentView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let IdentView { ident } = view_param;
        let data = match ident.get_data(tree) {
            Ok(data) => data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        ident.0,
                        NodeKind::Terminal(ident.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_ident_terminal(ident, data, tree)?;
        Ok(())
    }
    fn visit_in_str_super(
        &mut self,
        handle: InStrHandle,
        view_param: InStrView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let InStrView { in_str } = view_param;
        let data = match in_str.get_data(tree) {
            Ok(data) => data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        in_str.0,
                        NodeKind::Terminal(in_str.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_in_str_terminal(in_str, data, tree)?;
        Ok(())
    }
    fn visit_integer_super(
        &mut self,
        handle: IntegerHandle,
        view_param: IntegerView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let IntegerView { integer } = view_param;
        let data = match integer.get_data(tree) {
            Ok(data) => data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        integer.0,
                        NodeKind::Terminal(integer.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_integer_terminal(integer, data, tree)?;
        Ok(())
    }
    fn visit_key_super(
        &mut self,
        handle: KeyHandle,
        view_param: KeyView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let KeyView { key_base, key_opt } = view_param;
        self.visit_key_base_handle(key_base, tree)?;
        self.visit_key_opt_handle(key_opt, tree)?;
        Ok(())
    }
    fn visit_key_base_super(
        &mut self,
        handle: KeyBaseHandle,
        view_param: KeyBaseView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        match view_param {
            KeyBaseView::Ident(item) => {
                self.visit_ident_handle(item, tree)?;
            }
            KeyBaseView::ExtensionNameSpace(item) => {
                self.visit_extension_name_space_handle(item, tree)?;
            }
            KeyBaseView::Str(item) => {
                self.visit_str_handle(item, tree)?;
            }
            KeyBaseView::Integer(item) => {
                self.visit_integer_handle(item, tree)?;
            }
        }
        Ok(())
    }
    fn visit_key_opt_super(
        &mut self,
        handle: KeyOptHandle,
        view_param: ArrayMarkerHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        self.visit_array_marker_handle(view_param, tree)?;
        Ok(())
    }
    fn visit_keys_super(
        &mut self,
        handle: KeysHandle,
        view_param: KeysView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let KeysView { key, keys_list } = view_param;
        self.visit_key_handle(key, tree)?;
        self.visit_keys_list_handle(keys_list, tree)?;
        Ok(())
    }
    fn visit_keys_list_super(
        &mut self,
        handle: KeysListHandle,
        view_param: KeysListView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let KeysListView { dot, key, keys_list } = view_param;
        self.visit_dot_handle(dot, tree)?;
        self.visit_key_handle(key, tree)?;
        self.visit_keys_list_handle(keys_list, tree)?;
        Ok(())
    }
    fn visit_named_code_super(
        &mut self,
        handle: NamedCodeHandle,
        view_param: NamedCodeView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let NamedCodeView { named_code } = view_param;
        let data = match named_code.get_data(tree) {
            Ok(data) => data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        named_code.0,
                        NodeKind::Terminal(named_code.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_named_code_terminal(named_code, data, tree)?;
        Ok(())
    }
    fn visit_newline_super(
        &mut self,
        handle: NewlineHandle,
        view_param: NewlineView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let NewlineView { newline } = view_param;
        let data = match newline.get_data(tree) {
            Ok(data) => data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        newline.0,
                        NodeKind::Terminal(newline.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_newline_terminal(newline, data, tree)?;
        Ok(())
    }
    fn visit_null_super(
        &mut self,
        handle: NullHandle,
        view_param: NullView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let NullView { null } = view_param;
        let data = match null.get_data(tree) {
            Ok(data) => data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        null.0,
                        NodeKind::Terminal(null.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_null_terminal(null, data, tree)?;
        Ok(())
    }
    fn visit_object_super(
        &mut self,
        handle: ObjectHandle,
        view_param: ObjectView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let ObjectView { begin, object_list, end } = view_param;
        self.visit_begin_handle(begin, tree)?;
        self.visit_object_list_handle(object_list, tree)?;
        self.visit_end_handle(end, tree)?;
        Ok(())
    }
    fn visit_object_list_super(
        &mut self,
        handle: ObjectListHandle,
        view_param: ObjectListView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let ObjectListView { key, bind, value, object_opt, object_list } = view_param;
        self.visit_key_handle(key, tree)?;
        self.visit_bind_handle(bind, tree)?;
        self.visit_value_handle(value, tree)?;
        self.visit_object_opt_handle(object_opt, tree)?;
        self.visit_object_list_handle(object_list, tree)?;
        Ok(())
    }
    fn visit_object_opt_super(
        &mut self,
        handle: ObjectOptHandle,
        view_param: CommaHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        self.visit_comma_handle(view_param, tree)?;
        Ok(())
    }
    fn visit_quote_super(
        &mut self,
        handle: QuoteHandle,
        view_param: QuoteView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let QuoteView { quote } = view_param;
        let data = match quote.get_data(tree) {
            Ok(data) => data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        quote.0,
                        NodeKind::Terminal(quote.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_quote_terminal(quote, data, tree)?;
        Ok(())
    }
    fn visit_section_super(
        &mut self,
        handle: SectionHandle,
        view_param: SectionView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let SectionView { at, keys, section_body } = view_param;
        self.visit_at_handle(at, tree)?;
        self.visit_keys_handle(keys, tree)?;
        self.visit_section_body_handle(section_body, tree)?;
        Ok(())
    }
    fn visit_section_binding_super(
        &mut self,
        handle: SectionBindingHandle,
        view_param: SectionBindingView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let SectionBindingView { begin, swon, end } = view_param;
        self.visit_begin_handle(begin, tree)?;
        self.visit_swon_handle(swon, tree)?;
        self.visit_end_handle(end, tree)?;
        Ok(())
    }
    fn visit_section_body_super(
        &mut self,
        handle: SectionBodyHandle,
        view_param: SectionBodyView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        match view_param {
            SectionBodyView::SectionBodyList(item) => {
                self.visit_section_body_list_handle(item, tree)?;
            }
            SectionBodyView::SectionBinding(item) => {
                self.visit_section_binding_handle(item, tree)?;
            }
        }
        Ok(())
    }
    fn visit_section_body_list_super(
        &mut self,
        handle: SectionBodyListHandle,
        view_param: SectionBodyListView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let SectionBodyListView { binding, section_body_list } = view_param;
        self.visit_binding_handle(binding, tree)?;
        self.visit_section_body_list_handle(section_body_list, tree)?;
        Ok(())
    }
    fn visit_str_super(
        &mut self,
        handle: StrHandle,
        view_param: StrView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let StrView { quote, in_str, quote2 } = view_param;
        self.visit_quote_handle(quote, tree)?;
        self.visit_in_str_handle(in_str, tree)?;
        self.visit_quote_handle(quote2, tree)?;
        Ok(())
    }
    fn visit_str_continues_super(
        &mut self,
        handle: StrContinuesHandle,
        view_param: StrContinuesView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let StrContinuesView { str, str_continues_list } = view_param;
        self.visit_str_handle(str, tree)?;
        self.visit_str_continues_list_handle(str_continues_list, tree)?;
        Ok(())
    }
    fn visit_str_continues_list_super(
        &mut self,
        handle: StrContinuesListHandle,
        view_param: StrContinuesListView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let StrContinuesListView { r#continue, str, str_continues_list } = view_param;
        self.visit_continue_handle(r#continue, tree)?;
        self.visit_str_handle(str, tree)?;
        self.visit_str_continues_list_handle(str_continues_list, tree)?;
        Ok(())
    }
    fn visit_swon_super(
        &mut self,
        handle: SwonHandle,
        view_param: SwonView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let SwonView { swon_bindings, swon_sections } = view_param;
        self.visit_swon_bindings_handle(swon_bindings, tree)?;
        self.visit_swon_sections_handle(swon_sections, tree)?;
        Ok(())
    }
    fn visit_swon_bindings_super(
        &mut self,
        handle: SwonBindingsHandle,
        view_param: SwonBindingsView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let SwonBindingsView { binding, swon_bindings } = view_param;
        self.visit_binding_handle(binding, tree)?;
        self.visit_swon_bindings_handle(swon_bindings, tree)?;
        Ok(())
    }
    fn visit_swon_sections_super(
        &mut self,
        handle: SwonSectionsHandle,
        view_param: SwonSectionsView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let SwonSectionsView { section, swon_sections } = view_param;
        self.visit_section_handle(section, tree)?;
        self.visit_swon_sections_handle(swon_sections, tree)?;
        Ok(())
    }
    fn visit_text_super(
        &mut self,
        handle: TextHandle,
        view_param: TextView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let TextView { text } = view_param;
        let data = match text.get_data(tree) {
            Ok(data) => data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        text.0,
                        NodeKind::Terminal(text.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_text_terminal(text, data, tree)?;
        Ok(())
    }
    fn visit_text_binding_super(
        &mut self,
        handle: TextBindingHandle,
        view_param: TextBindingView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let TextBindingView { text_start, text_binding_opt, text, newline } = view_param;
        self.visit_text_start_handle(text_start, tree)?;
        self.visit_text_binding_opt_handle(text_binding_opt, tree)?;
        self.visit_text_handle(text, tree)?;
        self.visit_newline_handle(newline, tree)?;
        Ok(())
    }
    fn visit_text_binding_opt_super(
        &mut self,
        handle: TextBindingOptHandle,
        view_param: WsHandle,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        self.visit_ws_handle(view_param, tree)?;
        Ok(())
    }
    fn visit_text_start_super(
        &mut self,
        handle: TextStartHandle,
        view_param: TextStartView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let TextStartView { text_start } = view_param;
        let data = match text_start.get_data(tree) {
            Ok(data) => data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        text_start.0,
                        NodeKind::Terminal(text_start.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_text_start_terminal(text_start, data, tree)?;
        Ok(())
    }
    fn visit_true_super(
        &mut self,
        handle: TrueHandle,
        view_param: TrueView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let TrueView { r#true } = view_param;
        let data = match r#true.get_data(tree) {
            Ok(data) => data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        r#true.0,
                        NodeKind::Terminal(r#true.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_true_terminal(r#true, data, tree)?;
        Ok(())
    }
    fn visit_typed_quote_super(
        &mut self,
        handle: TypedQuoteHandle,
        view_param: TypedQuoteView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let TypedQuoteView { typed_quote } = view_param;
        let data = match typed_quote.get_data(tree) {
            Ok(data) => data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        typed_quote.0,
                        NodeKind::Terminal(typed_quote.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_typed_quote_terminal(typed_quote, data, tree)?;
        Ok(())
    }
    fn visit_typed_str_super(
        &mut self,
        handle: TypedStrHandle,
        view_param: TypedStrView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let TypedStrView { typed_quote, in_str, quote } = view_param;
        self.visit_typed_quote_handle(typed_quote, tree)?;
        self.visit_in_str_handle(in_str, tree)?;
        self.visit_quote_handle(quote, tree)?;
        Ok(())
    }
    fn visit_value_super(
        &mut self,
        handle: ValueHandle,
        view_param: ValueView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        match view_param {
            ValueView::Object(item) => {
                self.visit_object_handle(item, tree)?;
            }
            ValueView::Array(item) => {
                self.visit_array_handle(item, tree)?;
            }
            ValueView::Integer(item) => {
                self.visit_integer_handle(item, tree)?;
            }
            ValueView::Boolean(item) => {
                self.visit_boolean_handle(item, tree)?;
            }
            ValueView::Null(item) => {
                self.visit_null_handle(item, tree)?;
            }
            ValueView::StrContinues(item) => {
                self.visit_str_continues_handle(item, tree)?;
            }
            ValueView::TypedStr(item) => {
                self.visit_typed_str_handle(item, tree)?;
            }
            ValueView::Hole(item) => {
                self.visit_hole_handle(item, tree)?;
            }
            ValueView::CodeBlock(item) => {
                self.visit_code_block_handle(item, tree)?;
            }
            ValueView::NamedCode(item) => {
                self.visit_named_code_handle(item, tree)?;
            }
            ValueView::Code(item) => {
                self.visit_code_handle(item, tree)?;
            }
        }
        Ok(())
    }
    fn visit_value_binding_super(
        &mut self,
        handle: ValueBindingHandle,
        view_param: ValueBindingView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let ValueBindingView { bind, value } = view_param;
        self.visit_bind_handle(bind, tree)?;
        self.visit_value_handle(value, tree)?;
        Ok(())
    }
    fn visit_ws_super(
        &mut self,
        handle: WsHandle,
        view_param: WsView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let WsView { ws } = view_param;
        let data = match ws.get_data(tree) {
            Ok(data) => data,
            Err(error) => {
                return self
                    .then_construct_error(
                        None,
                        ws.0,
                        NodeKind::Terminal(ws.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_ws_terminal(ws, data, tree)?;
        Ok(())
    }
    fn visit_root_super(
        &mut self,
        handle: RootHandle,
        view_param: RootView,
        tree: &F,
    ) -> Result<(), V::Error> {
        let _handle = handle;
        let RootView { swon } = view_param;
        self.visit_swon_handle(swon, tree)?;
        Ok(())
    }
    fn visit_new_line_terminal_super(
        &mut self,
        terminal: NewLine,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_whitespace_terminal_super(
        &mut self,
        terminal: Whitespace,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_line_comment_terminal_super(
        &mut self,
        terminal: LineComment,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_block_comment_terminal_super(
        &mut self,
        terminal: BlockComment,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_integer_terminal_super(
        &mut self,
        terminal: Integer,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_true_terminal_super(
        &mut self,
        terminal: True,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_false_terminal_super(
        &mut self,
        terminal: False,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_null_terminal_super(
        &mut self,
        terminal: Null,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_hole_terminal_super(
        &mut self,
        terminal: Hole,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_quote_terminal_super(
        &mut self,
        terminal: Quote,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_typed_quote_terminal_super(
        &mut self,
        terminal: TypedQuote,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_in_str_terminal_super(
        &mut self,
        terminal: InStr,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_text_terminal_super(
        &mut self,
        terminal: Text,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_code_block_terminal_super(
        &mut self,
        terminal: CodeBlock,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_named_code_terminal_super(
        &mut self,
        terminal: NamedCode,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_code_terminal_super(
        &mut self,
        terminal: Code,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_newline_terminal_super(
        &mut self,
        terminal: Newline,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_ws_terminal_super(
        &mut self,
        terminal: Ws,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_at_terminal_super(
        &mut self,
        terminal: At,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_dollar_terminal_super(
        &mut self,
        terminal: Dollar,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_dot_terminal_super(
        &mut self,
        terminal: Dot,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_l_brace_terminal_super(
        &mut self,
        terminal: LBrace,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_r_brace_terminal_super(
        &mut self,
        terminal: RBrace,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_l_bracket_terminal_super(
        &mut self,
        terminal: LBracket,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_r_bracket_terminal_super(
        &mut self,
        terminal: RBracket,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_bind_terminal_super(
        &mut self,
        terminal: Bind,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_comma_terminal_super(
        &mut self,
        terminal: Comma,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_esc_terminal_super(
        &mut self,
        terminal: Esc,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_text_start_terminal_super(
        &mut self,
        terminal: TextStart,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_ident_terminal_super(
        &mut self,
        terminal: Ident,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_non_terminal_super(
        &mut self,
        _id: CstNodeId,
        _kind: NonTerminalKind,
        _data: NonTerminalData,
        _tree: &F,
    ) -> Result<(), V::Error> {
        Ok(())
    }
    fn visit_non_terminal_close_super(
        &mut self,
        _id: CstNodeId,
        _kind: NonTerminalKind,
        _data: NonTerminalData,
        _tree: &F,
    ) -> Result<(), V::Error> {
        Ok(())
    }
    fn visit_terminal_super(
        &mut self,
        _id: CstNodeId,
        _kind: TerminalKind,
        _data: TerminalData,
        _tree: &F,
    ) -> Result<(), V::Error> {
        Ok(())
    }
    fn recover_error(
        &mut self,
        node_data: Option<CstNode>,
        id: CstNodeId,
        kind: NodeKind,
        tree: &F,
    ) -> Result<(), V::Error> {
        let Some(node_data) = node_data else {
            return Ok(());
        };
        if node_data.node_kind() == kind {
            for child in tree.children(id) {
                if let Some(node_data) = tree.node_data(child) {
                    self.visit_any(child, node_data, tree)?;
                }
            }
        } else {
            self.visit_any(id, node_data, tree)?;
        }
        Ok(())
    }
    fn visit_any(
        &mut self,
        id: CstNodeId,
        node: CstNode,
        tree: &F,
    ) -> Result<(), V::Error> {
        match node {
            CstNode::NonTerminal { kind, .. } => {
                match kind {
                    NonTerminalKind::Array => {
                        let handle = ArrayHandle(id);
                        self.visit_array_handle(handle, tree)?;
                    }
                    NonTerminalKind::ArrayBegin => {
                        let handle = ArrayBeginHandle(id);
                        self.visit_array_begin_handle(handle, tree)?;
                    }
                    NonTerminalKind::ArrayEnd => {
                        let handle = ArrayEndHandle(id);
                        self.visit_array_end_handle(handle, tree)?;
                    }
                    NonTerminalKind::ArrayList => {
                        let handle = ArrayListHandle(id);
                        self.visit_array_list_handle(handle, tree)?;
                    }
                    NonTerminalKind::ArrayMarker => {
                        let handle = ArrayMarkerHandle(id);
                        self.visit_array_marker_handle(handle, tree)?;
                    }
                    NonTerminalKind::ArrayMarkerOpt => {
                        let handle = ArrayMarkerOptHandle(id);
                        self.visit_array_marker_opt_handle(handle, tree)?;
                    }
                    NonTerminalKind::ArrayOpt => {
                        let handle = ArrayOptHandle(id);
                        self.visit_array_opt_handle(handle, tree)?;
                    }
                    NonTerminalKind::At => {
                        let handle = AtHandle(id);
                        self.visit_at_handle(handle, tree)?;
                    }
                    NonTerminalKind::Begin => {
                        let handle = BeginHandle(id);
                        self.visit_begin_handle(handle, tree)?;
                    }
                    NonTerminalKind::Bind => {
                        let handle = BindHandle(id);
                        self.visit_bind_handle(handle, tree)?;
                    }
                    NonTerminalKind::Binding => {
                        let handle = BindingHandle(id);
                        self.visit_binding_handle(handle, tree)?;
                    }
                    NonTerminalKind::BindingRhs => {
                        let handle = BindingRhsHandle(id);
                        self.visit_binding_rhs_handle(handle, tree)?;
                    }
                    NonTerminalKind::Boolean => {
                        let handle = BooleanHandle(id);
                        self.visit_boolean_handle(handle, tree)?;
                    }
                    NonTerminalKind::Code => {
                        let handle = CodeHandle(id);
                        self.visit_code_handle(handle, tree)?;
                    }
                    NonTerminalKind::CodeBlock => {
                        let handle = CodeBlockHandle(id);
                        self.visit_code_block_handle(handle, tree)?;
                    }
                    NonTerminalKind::Comma => {
                        let handle = CommaHandle(id);
                        self.visit_comma_handle(handle, tree)?;
                    }
                    NonTerminalKind::Continue => {
                        let handle = ContinueHandle(id);
                        self.visit_continue_handle(handle, tree)?;
                    }
                    NonTerminalKind::Dot => {
                        let handle = DotHandle(id);
                        self.visit_dot_handle(handle, tree)?;
                    }
                    NonTerminalKind::End => {
                        let handle = EndHandle(id);
                        self.visit_end_handle(handle, tree)?;
                    }
                    NonTerminalKind::Ext => {
                        let handle = ExtHandle(id);
                        self.visit_ext_handle(handle, tree)?;
                    }
                    NonTerminalKind::ExtensionNameSpace => {
                        let handle = ExtensionNameSpaceHandle(id);
                        self.visit_extension_name_space_handle(handle, tree)?;
                    }
                    NonTerminalKind::False => {
                        let handle = FalseHandle(id);
                        self.visit_false_handle(handle, tree)?;
                    }
                    NonTerminalKind::Hole => {
                        let handle = HoleHandle(id);
                        self.visit_hole_handle(handle, tree)?;
                    }
                    NonTerminalKind::Ident => {
                        let handle = IdentHandle(id);
                        self.visit_ident_handle(handle, tree)?;
                    }
                    NonTerminalKind::InStr => {
                        let handle = InStrHandle(id);
                        self.visit_in_str_handle(handle, tree)?;
                    }
                    NonTerminalKind::Integer => {
                        let handle = IntegerHandle(id);
                        self.visit_integer_handle(handle, tree)?;
                    }
                    NonTerminalKind::Key => {
                        let handle = KeyHandle(id);
                        self.visit_key_handle(handle, tree)?;
                    }
                    NonTerminalKind::KeyBase => {
                        let handle = KeyBaseHandle(id);
                        self.visit_key_base_handle(handle, tree)?;
                    }
                    NonTerminalKind::KeyOpt => {
                        let handle = KeyOptHandle(id);
                        self.visit_key_opt_handle(handle, tree)?;
                    }
                    NonTerminalKind::Keys => {
                        let handle = KeysHandle(id);
                        self.visit_keys_handle(handle, tree)?;
                    }
                    NonTerminalKind::KeysList => {
                        let handle = KeysListHandle(id);
                        self.visit_keys_list_handle(handle, tree)?;
                    }
                    NonTerminalKind::NamedCode => {
                        let handle = NamedCodeHandle(id);
                        self.visit_named_code_handle(handle, tree)?;
                    }
                    NonTerminalKind::Newline => {
                        let handle = NewlineHandle(id);
                        self.visit_newline_handle(handle, tree)?;
                    }
                    NonTerminalKind::Null => {
                        let handle = NullHandle(id);
                        self.visit_null_handle(handle, tree)?;
                    }
                    NonTerminalKind::Object => {
                        let handle = ObjectHandle(id);
                        self.visit_object_handle(handle, tree)?;
                    }
                    NonTerminalKind::ObjectList => {
                        let handle = ObjectListHandle(id);
                        self.visit_object_list_handle(handle, tree)?;
                    }
                    NonTerminalKind::ObjectOpt => {
                        let handle = ObjectOptHandle(id);
                        self.visit_object_opt_handle(handle, tree)?;
                    }
                    NonTerminalKind::Quote => {
                        let handle = QuoteHandle(id);
                        self.visit_quote_handle(handle, tree)?;
                    }
                    NonTerminalKind::Section => {
                        let handle = SectionHandle(id);
                        self.visit_section_handle(handle, tree)?;
                    }
                    NonTerminalKind::SectionBinding => {
                        let handle = SectionBindingHandle(id);
                        self.visit_section_binding_handle(handle, tree)?;
                    }
                    NonTerminalKind::SectionBody => {
                        let handle = SectionBodyHandle(id);
                        self.visit_section_body_handle(handle, tree)?;
                    }
                    NonTerminalKind::SectionBodyList => {
                        let handle = SectionBodyListHandle(id);
                        self.visit_section_body_list_handle(handle, tree)?;
                    }
                    NonTerminalKind::Str => {
                        let handle = StrHandle(id);
                        self.visit_str_handle(handle, tree)?;
                    }
                    NonTerminalKind::StrContinues => {
                        let handle = StrContinuesHandle(id);
                        self.visit_str_continues_handle(handle, tree)?;
                    }
                    NonTerminalKind::StrContinuesList => {
                        let handle = StrContinuesListHandle(id);
                        self.visit_str_continues_list_handle(handle, tree)?;
                    }
                    NonTerminalKind::Swon => {
                        let handle = SwonHandle(id);
                        self.visit_swon_handle(handle, tree)?;
                    }
                    NonTerminalKind::SwonList => {
                        let handle = SwonBindingsHandle(id);
                        self.visit_swon_bindings_handle(handle, tree)?;
                    }
                    NonTerminalKind::SwonList0 => {
                        let handle = SwonSectionsHandle(id);
                        self.visit_swon_sections_handle(handle, tree)?;
                    }
                    NonTerminalKind::Text => {
                        let handle = TextHandle(id);
                        self.visit_text_handle(handle, tree)?;
                    }
                    NonTerminalKind::TextBinding => {
                        let handle = TextBindingHandle(id);
                        self.visit_text_binding_handle(handle, tree)?;
                    }
                    NonTerminalKind::TextBindingOpt => {
                        let handle = TextBindingOptHandle(id);
                        self.visit_text_binding_opt_handle(handle, tree)?;
                    }
                    NonTerminalKind::TextStart => {
                        let handle = TextStartHandle(id);
                        self.visit_text_start_handle(handle, tree)?;
                    }
                    NonTerminalKind::True => {
                        let handle = TrueHandle(id);
                        self.visit_true_handle(handle, tree)?;
                    }
                    NonTerminalKind::TypedQuote => {
                        let handle = TypedQuoteHandle(id);
                        self.visit_typed_quote_handle(handle, tree)?;
                    }
                    NonTerminalKind::TypedStr => {
                        let handle = TypedStrHandle(id);
                        self.visit_typed_str_handle(handle, tree)?;
                    }
                    NonTerminalKind::Value => {
                        let handle = ValueHandle(id);
                        self.visit_value_handle(handle, tree)?;
                    }
                    NonTerminalKind::ValueBinding => {
                        let handle = ValueBindingHandle(id);
                        self.visit_value_binding_handle(handle, tree)?;
                    }
                    NonTerminalKind::Ws => {
                        let handle = WsHandle(id);
                        self.visit_ws_handle(handle, tree)?;
                    }
                    NonTerminalKind::Root => {
                        let handle = RootHandle(id);
                        self.visit_root_handle(handle, tree)?;
                    }
                }
            }
            CstNode::Terminal { kind, data } => {
                match kind {
                    TerminalKind::NewLine => {
                        let terminal = NewLine(id);
                        self.visit_new_line_terminal(terminal, data, tree)?;
                    }
                    TerminalKind::Whitespace => {
                        let terminal = Whitespace(id);
                        self.visit_whitespace_terminal(terminal, data, tree)?;
                    }
                    TerminalKind::LineComment => {
                        let terminal = LineComment(id);
                        self.visit_line_comment_terminal(terminal, data, tree)?;
                    }
                    TerminalKind::BlockComment => {
                        let terminal = BlockComment(id);
                        self.visit_block_comment_terminal(terminal, data, tree)?;
                    }
                    TerminalKind::Integer => {
                        let terminal = Integer(id);
                        self.visit_integer_terminal(terminal, data, tree)?;
                    }
                    TerminalKind::True => {
                        let terminal = True(id);
                        self.visit_true_terminal(terminal, data, tree)?;
                    }
                    TerminalKind::False => {
                        let terminal = False(id);
                        self.visit_false_terminal(terminal, data, tree)?;
                    }
                    TerminalKind::Null => {
                        let terminal = Null(id);
                        self.visit_null_terminal(terminal, data, tree)?;
                    }
                    TerminalKind::Hole => {
                        let terminal = Hole(id);
                        self.visit_hole_terminal(terminal, data, tree)?;
                    }
                    TerminalKind::Quote => {
                        let terminal = Quote(id);
                        self.visit_quote_terminal(terminal, data, tree)?;
                    }
                    TerminalKind::TypedQuote => {
                        let terminal = TypedQuote(id);
                        self.visit_typed_quote_terminal(terminal, data, tree)?;
                    }
                    TerminalKind::InStr => {
                        let terminal = InStr(id);
                        self.visit_in_str_terminal(terminal, data, tree)?;
                    }
                    TerminalKind::Text => {
                        let terminal = Text(id);
                        self.visit_text_terminal(terminal, data, tree)?;
                    }
                    TerminalKind::CodeBlock => {
                        let terminal = CodeBlock(id);
                        self.visit_code_block_terminal(terminal, data, tree)?;
                    }
                    TerminalKind::NamedCode => {
                        let terminal = NamedCode(id);
                        self.visit_named_code_terminal(terminal, data, tree)?;
                    }
                    TerminalKind::Code => {
                        let terminal = Code(id);
                        self.visit_code_terminal(terminal, data, tree)?;
                    }
                    TerminalKind::Newline => {
                        let terminal = Newline(id);
                        self.visit_newline_terminal(terminal, data, tree)?;
                    }
                    TerminalKind::Ws => {
                        let terminal = Ws(id);
                        self.visit_ws_terminal(terminal, data, tree)?;
                    }
                    TerminalKind::At => {
                        let terminal = At(id);
                        self.visit_at_terminal(terminal, data, tree)?;
                    }
                    TerminalKind::Dollar => {
                        let terminal = Dollar(id);
                        self.visit_dollar_terminal(terminal, data, tree)?;
                    }
                    TerminalKind::Dot => {
                        let terminal = Dot(id);
                        self.visit_dot_terminal(terminal, data, tree)?;
                    }
                    TerminalKind::LBrace => {
                        let terminal = LBrace(id);
                        self.visit_l_brace_terminal(terminal, data, tree)?;
                    }
                    TerminalKind::RBrace => {
                        let terminal = RBrace(id);
                        self.visit_r_brace_terminal(terminal, data, tree)?;
                    }
                    TerminalKind::LBracket => {
                        let terminal = LBracket(id);
                        self.visit_l_bracket_terminal(terminal, data, tree)?;
                    }
                    TerminalKind::RBracket => {
                        let terminal = RBracket(id);
                        self.visit_r_bracket_terminal(terminal, data, tree)?;
                    }
                    TerminalKind::Bind => {
                        let terminal = Bind(id);
                        self.visit_bind_terminal(terminal, data, tree)?;
                    }
                    TerminalKind::Comma => {
                        let terminal = Comma(id);
                        self.visit_comma_terminal(terminal, data, tree)?;
                    }
                    TerminalKind::Esc => {
                        let terminal = Esc(id);
                        self.visit_esc_terminal(terminal, data, tree)?;
                    }
                    TerminalKind::TextStart => {
                        let terminal = TextStart(id);
                        self.visit_text_start_terminal(terminal, data, tree)?;
                    }
                    TerminalKind::Ident => {
                        let terminal = Ident(id);
                        self.visit_ident_terminal(terminal, data, tree)?;
                    }
                }
            }
        }
        Ok(())
    }
}
mod private2 {
    pub trait Sealed {}
}
pub trait NodeVisitor: NodeVisitorSuper<Self::Error> {
    type Error;
    fn visit_node(
        &mut self,
        id: CstNodeId,
        node: CstNode,
        tree: &Cst,
    ) -> Result<(), Self::Error>;
}
pub trait NodeVisitorSuper<E>: private2::Sealed {
    fn visit_node_id(&mut self, id: CstNodeId, tree: &Cst) -> Result<(), E>;
    fn visit_node_super(
        &mut self,
        id: CstNodeId,
        node: CstNode,
        tree: &Cst,
    ) -> Result<(), E>;
}
impl<V: NodeVisitor> private2::Sealed for V {}
impl<V: NodeVisitor> NodeVisitorSuper<V::Error> for V {
    fn visit_node_id(&mut self, id: CstNodeId, tree: &Cst) -> Result<(), V::Error> {
        if let Some(node) = tree.node_data(id) {
            self.visit_node(id, node, tree)
        } else {
            Ok(())
        }
    }
    fn visit_node_super(
        &mut self,
        id: CstNodeId,
        _node: CstNode,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        for child in tree.children(id) {
            if let Some(child_node) = tree.node_data(child) {
                self.visit_node(child, child_node, tree)?;
            }
        }
        Ok(())
    }
}
pub trait BuiltinTerminalVisitor<E, F: CstFacade> {
    fn visit_builtin_new_line_terminal(
        &mut self,
        terminal: NewLine,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_builtin_whitespace_terminal(
        &mut self,
        terminal: Whitespace,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_builtin_line_comment_terminal(
        &mut self,
        terminal: LineComment,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), E>;
    fn visit_builtin_block_comment_terminal(
        &mut self,
        terminal: BlockComment,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), E>;
}
impl<V: CstVisitor<F>, F: CstFacade> BuiltinTerminalVisitor<V::Error, F> for V {
    fn visit_builtin_new_line_terminal(
        &mut self,
        terminal: NewLine,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), V::Error> {
        self.visit_new_line_terminal(terminal, data, tree)
    }
    fn visit_builtin_whitespace_terminal(
        &mut self,
        terminal: Whitespace,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), V::Error> {
        self.visit_whitespace_terminal(terminal, data, tree)
    }
    fn visit_builtin_line_comment_terminal(
        &mut self,
        terminal: LineComment,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), V::Error> {
        self.visit_line_comment_terminal(terminal, data, tree)
    }
    fn visit_builtin_block_comment_terminal(
        &mut self,
        terminal: BlockComment,
        data: TerminalData,
        tree: &F,
    ) -> Result<(), V::Error> {
        self.visit_block_comment_terminal(terminal, data, tree)
    }
}
