use crate::{
    Cst, CstConstructError, NodeKind, CstNode, nodes::*,
    node_kind::{TerminalKind, NonTerminalKind},
    tree::{NonTerminalHandle as _, TerminalData, NonTerminalData, CstNodeId},
};
pub trait CstVisitor: CstHandleSuper<Self::Error> {
    type Error;
    fn visit_array(
        &mut self,
        handle: ArrayHandle,
        view: ArrayView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_array_super(view, tree)
    }
    fn visit_array_begin(
        &mut self,
        handle: ArrayBeginHandle,
        view: ArrayBeginView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_array_begin_super(view, tree)
    }
    fn visit_array_end(
        &mut self,
        handle: ArrayEndHandle,
        view: ArrayEndView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_array_end_super(view, tree)
    }
    fn visit_array_list(
        &mut self,
        handle: ArrayListHandle,
        view: ArrayListView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_array_list_super(view, tree)
    }
    fn visit_array_marker(
        &mut self,
        handle: ArrayMarkerHandle,
        view: ArrayMarkerView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_array_marker_super(view, tree)
    }
    fn visit_array_marker_opt(
        &mut self,
        handle: ArrayMarkerOptHandle,
        view: IntegerHandle,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_array_marker_opt_super(view, tree)
    }
    fn visit_array_opt(
        &mut self,
        handle: ArrayOptHandle,
        view: CommaHandle,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_array_opt_super(view, tree)
    }
    fn visit_at(
        &mut self,
        handle: AtHandle,
        view: AtView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_at_super(view, tree)
    }
    fn visit_begin(
        &mut self,
        handle: BeginHandle,
        view: BeginView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_begin_super(view, tree)
    }
    fn visit_bind(
        &mut self,
        handle: BindHandle,
        view: BindView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_bind_super(view, tree)
    }
    fn visit_binding(
        &mut self,
        handle: BindingHandle,
        view: BindingView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_binding_super(view, tree)
    }
    fn visit_binding_rhs(
        &mut self,
        handle: BindingRhsHandle,
        view: BindingRhsView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_binding_rhs_super(view, tree)
    }
    fn visit_boolean(
        &mut self,
        handle: BooleanHandle,
        view: BooleanView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_boolean_super(view, tree)
    }
    fn visit_code(
        &mut self,
        handle: CodeHandle,
        view: CodeView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_code_super(view, tree)
    }
    fn visit_code_block(
        &mut self,
        handle: CodeBlockHandle,
        view: CodeBlockView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_code_block_super(view, tree)
    }
    fn visit_code_block_delimiter(
        &mut self,
        handle: CodeBlockDelimiterHandle,
        view: CodeBlockDelimiterView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_code_block_delimiter_super(view, tree)
    }
    fn visit_code_block_line(
        &mut self,
        handle: CodeBlockLineHandle,
        view: CodeBlockLineView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_code_block_line_super(view, tree)
    }
    fn visit_code_block_tail_common(
        &mut self,
        handle: CodeBlockTailCommonHandle,
        view: CodeBlockTailCommonView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_code_block_tail_common_super(view, tree)
    }
    fn visit_code_block_tail_common_list(
        &mut self,
        handle: CodeBlockTailCommonListHandle,
        view: CodeBlockTailCommonListView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_code_block_tail_common_list_super(view, tree)
    }
    fn visit_code_block_tail_common_opt(
        &mut self,
        handle: CodeBlockTailCommonOptHandle,
        view: WsHandle,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_code_block_tail_common_opt_super(view, tree)
    }
    fn visit_comma(
        &mut self,
        handle: CommaHandle,
        view: CommaView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_comma_super(view, tree)
    }
    fn visit_continue(
        &mut self,
        handle: ContinueHandle,
        view: ContinueView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_continue_super(view, tree)
    }
    fn visit_dot(
        &mut self,
        handle: DotHandle,
        view: DotView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_dot_super(view, tree)
    }
    fn visit_end(
        &mut self,
        handle: EndHandle,
        view: EndView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_end_super(view, tree)
    }
    fn visit_ext(
        &mut self,
        handle: ExtHandle,
        view: ExtView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_ext_super(view, tree)
    }
    fn visit_extension_name_space(
        &mut self,
        handle: ExtensionNameSpaceHandle,
        view: ExtensionNameSpaceView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_extension_name_space_super(view, tree)
    }
    fn visit_false(
        &mut self,
        handle: FalseHandle,
        view: FalseView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_false_super(view, tree)
    }
    fn visit_hole(
        &mut self,
        handle: HoleHandle,
        view: HoleView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_hole_super(view, tree)
    }
    fn visit_ident(
        &mut self,
        handle: IdentHandle,
        view: IdentView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_ident_super(view, tree)
    }
    fn visit_in_str(
        &mut self,
        handle: InStrHandle,
        view: InStrView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_in_str_super(view, tree)
    }
    fn visit_integer(
        &mut self,
        handle: IntegerHandle,
        view: IntegerView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_integer_super(view, tree)
    }
    fn visit_key(
        &mut self,
        handle: KeyHandle,
        view: KeyView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_key_super(view, tree)
    }
    fn visit_key_base(
        &mut self,
        handle: KeyBaseHandle,
        view: KeyBaseView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_key_base_super(view, tree)
    }
    fn visit_key_opt(
        &mut self,
        handle: KeyOptHandle,
        view: ArrayMarkerHandle,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_key_opt_super(view, tree)
    }
    fn visit_keys(
        &mut self,
        handle: KeysHandle,
        view: KeysView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_keys_super(view, tree)
    }
    fn visit_keys_list(
        &mut self,
        handle: KeysListHandle,
        view: KeysListView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_keys_list_super(view, tree)
    }
    fn visit_named_code(
        &mut self,
        handle: NamedCodeHandle,
        view: NamedCodeView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_named_code_super(view, tree)
    }
    fn visit_named_code_block(
        &mut self,
        handle: NamedCodeBlockHandle,
        view: NamedCodeBlockView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_named_code_block_super(view, tree)
    }
    fn visit_named_code_block_begin(
        &mut self,
        handle: NamedCodeBlockBeginHandle,
        view: NamedCodeBlockBeginView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_named_code_block_begin_super(view, tree)
    }
    fn visit_newline(
        &mut self,
        handle: NewlineHandle,
        view: NewlineView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_newline_super(view, tree)
    }
    fn visit_null(
        &mut self,
        handle: NullHandle,
        view: NullView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_null_super(view, tree)
    }
    fn visit_object(
        &mut self,
        handle: ObjectHandle,
        view: ObjectView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_object_super(view, tree)
    }
    fn visit_object_list(
        &mut self,
        handle: ObjectListHandle,
        view: ObjectListView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_object_list_super(view, tree)
    }
    fn visit_object_opt(
        &mut self,
        handle: ObjectOptHandle,
        view: CommaHandle,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_object_opt_super(view, tree)
    }
    fn visit_quote(
        &mut self,
        handle: QuoteHandle,
        view: QuoteView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_quote_super(view, tree)
    }
    fn visit_section(
        &mut self,
        handle: SectionHandle,
        view: SectionView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_section_super(view, tree)
    }
    fn visit_section_binding(
        &mut self,
        handle: SectionBindingHandle,
        view: SectionBindingView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_section_binding_super(view, tree)
    }
    fn visit_section_body(
        &mut self,
        handle: SectionBodyHandle,
        view: SectionBodyView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_section_body_super(view, tree)
    }
    fn visit_section_body_list(
        &mut self,
        handle: SectionBodyListHandle,
        view: SectionBodyListView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_section_body_list_super(view, tree)
    }
    fn visit_str(
        &mut self,
        handle: StrHandle,
        view: StrView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_str_super(view, tree)
    }
    fn visit_str_continues(
        &mut self,
        handle: StrContinuesHandle,
        view: StrContinuesView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_str_continues_super(view, tree)
    }
    fn visit_str_continues_list(
        &mut self,
        handle: StrContinuesListHandle,
        view: StrContinuesListView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_str_continues_list_super(view, tree)
    }
    fn visit_swon(
        &mut self,
        handle: SwonHandle,
        view: SwonView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_swon_super(view, tree)
    }
    fn visit_swon_bindings(
        &mut self,
        handle: SwonBindingsHandle,
        view: SwonBindingsView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_swon_bindings_super(view, tree)
    }
    fn visit_swon_sections(
        &mut self,
        handle: SwonSectionsHandle,
        view: SwonSectionsView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_swon_sections_super(view, tree)
    }
    fn visit_text(
        &mut self,
        handle: TextHandle,
        view: TextView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_text_super(view, tree)
    }
    fn visit_text_binding(
        &mut self,
        handle: TextBindingHandle,
        view: TextBindingView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_text_binding_super(view, tree)
    }
    fn visit_text_binding_opt(
        &mut self,
        handle: TextBindingOptHandle,
        view: WsHandle,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_text_binding_opt_super(view, tree)
    }
    fn visit_text_start(
        &mut self,
        handle: TextStartHandle,
        view: TextStartView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_text_start_super(view, tree)
    }
    fn visit_true(
        &mut self,
        handle: TrueHandle,
        view: TrueView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_true_super(view, tree)
    }
    fn visit_typed_quote(
        &mut self,
        handle: TypedQuoteHandle,
        view: TypedQuoteView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_typed_quote_super(view, tree)
    }
    fn visit_typed_str(
        &mut self,
        handle: TypedStrHandle,
        view: TypedStrView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_typed_str_super(view, tree)
    }
    fn visit_value(
        &mut self,
        handle: ValueHandle,
        view: ValueView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_value_super(view, tree)
    }
    fn visit_value_binding(
        &mut self,
        handle: ValueBindingHandle,
        view: ValueBindingView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_value_binding_super(view, tree)
    }
    fn visit_ws(
        &mut self,
        handle: WsHandle,
        view: WsView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_ws_super(view, tree)
    }
    fn visit_root(
        &mut self,
        handle: RootHandle,
        view: RootView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_root_super(view, tree)
    }
    fn visit_new_line_terminal(
        &mut self,
        terminal: NewLine,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_new_line_terminal_super(terminal, data, tree)
    }
    fn visit_whitespace_terminal(
        &mut self,
        terminal: Whitespace,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_whitespace_terminal_super(terminal, data, tree)
    }
    fn visit_line_comment_terminal(
        &mut self,
        terminal: LineComment,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_line_comment_terminal_super(terminal, data, tree)
    }
    fn visit_block_comment_terminal(
        &mut self,
        terminal: BlockComment,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_block_comment_terminal_super(terminal, data, tree)
    }
    fn visit_integer_terminal(
        &mut self,
        terminal: Integer,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_integer_terminal_super(terminal, data, tree)
    }
    fn visit_true_terminal(
        &mut self,
        terminal: True,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_true_terminal_super(terminal, data, tree)
    }
    fn visit_false_terminal(
        &mut self,
        terminal: False,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_false_terminal_super(terminal, data, tree)
    }
    fn visit_null_terminal(
        &mut self,
        terminal: Null,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_null_terminal_super(terminal, data, tree)
    }
    fn visit_hole_terminal(
        &mut self,
        terminal: Hole,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_hole_terminal_super(terminal, data, tree)
    }
    fn visit_quote_terminal(
        &mut self,
        terminal: Quote,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_quote_terminal_super(terminal, data, tree)
    }
    fn visit_typed_quote_terminal(
        &mut self,
        terminal: TypedQuote,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_typed_quote_terminal_super(terminal, data, tree)
    }
    fn visit_in_str_terminal(
        &mut self,
        terminal: InStr,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_in_str_terminal_super(terminal, data, tree)
    }
    fn visit_text_terminal(
        &mut self,
        terminal: Text,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_text_terminal_super(terminal, data, tree)
    }
    fn visit_named_code_terminal(
        &mut self,
        terminal: NamedCode,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_named_code_terminal_super(terminal, data, tree)
    }
    fn visit_code_terminal(
        &mut self,
        terminal: Code,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_code_terminal_super(terminal, data, tree)
    }
    fn visit_newline_terminal(
        &mut self,
        terminal: Newline,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_newline_terminal_super(terminal, data, tree)
    }
    fn visit_ws_terminal(
        &mut self,
        terminal: Ws,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_ws_terminal_super(terminal, data, tree)
    }
    fn visit_at_terminal(
        &mut self,
        terminal: At,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_at_terminal_super(terminal, data, tree)
    }
    fn visit_dollar_terminal(
        &mut self,
        terminal: Dollar,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_dollar_terminal_super(terminal, data, tree)
    }
    fn visit_dot_terminal(
        &mut self,
        terminal: Dot,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_dot_terminal_super(terminal, data, tree)
    }
    fn visit_l_brace_terminal(
        &mut self,
        terminal: LBrace,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_l_brace_terminal_super(terminal, data, tree)
    }
    fn visit_r_brace_terminal(
        &mut self,
        terminal: RBrace,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_r_brace_terminal_super(terminal, data, tree)
    }
    fn visit_l_bracket_terminal(
        &mut self,
        terminal: LBracket,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_l_bracket_terminal_super(terminal, data, tree)
    }
    fn visit_r_bracket_terminal(
        &mut self,
        terminal: RBracket,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_r_bracket_terminal_super(terminal, data, tree)
    }
    fn visit_bind_terminal(
        &mut self,
        terminal: Bind,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_bind_terminal_super(terminal, data, tree)
    }
    fn visit_comma_terminal(
        &mut self,
        terminal: Comma,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_comma_terminal_super(terminal, data, tree)
    }
    fn visit_esc_terminal(
        &mut self,
        terminal: Esc,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_esc_terminal_super(terminal, data, tree)
    }
    fn visit_text_start_terminal(
        &mut self,
        terminal: TextStart,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_text_start_terminal_super(terminal, data, tree)
    }
    fn visit_ident_terminal(
        &mut self,
        terminal: Ident,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_ident_terminal_super(terminal, data, tree)
    }
    fn visit_named_code_block_begin_terminal(
        &mut self,
        terminal: NamedCodeBlockBegin,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_named_code_block_begin_terminal_super(terminal, data, tree)
    }
    fn visit_code_block_delimiter_terminal(
        &mut self,
        terminal: CodeBlockDelimiter,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_code_block_delimiter_terminal_super(terminal, data, tree)
    }
    fn visit_code_block_line_terminal(
        &mut self,
        terminal: CodeBlockLine,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_code_block_line_terminal_super(terminal, data, tree)
    }
    fn visit_non_terminal(
        &mut self,
        id: CstNodeId,
        kind: NonTerminalKind,
        data: NonTerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_non_terminal_super(id, kind, data, tree)
    }
    fn visit_non_terminal_close(
        &mut self,
        id: CstNodeId,
        kind: NonTerminalKind,
        data: NonTerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_non_terminal_close_super(id, kind, data, tree)
    }
    fn visit_terminal(
        &mut self,
        id: CstNodeId,
        kind: TerminalKind,
        data: TerminalData,
        tree: &Cst,
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
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _error = error;
        self.recover_error(node_data, parent, kind, tree)
    }
}
mod private {
    pub trait Sealed {}
}
pub trait CstHandleSuper<E>: private::Sealed {
    fn visit_array_handle(&mut self, handle: ArrayHandle, tree: &Cst) -> Result<(), E>;
    fn visit_array_super(&mut self, view: ArrayView, tree: &Cst) -> Result<(), E>;
    fn visit_array_begin_handle(
        &mut self,
        handle: ArrayBeginHandle,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_array_begin_super(
        &mut self,
        view: ArrayBeginView,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_array_end_handle(
        &mut self,
        handle: ArrayEndHandle,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_array_end_super(&mut self, view: ArrayEndView, tree: &Cst) -> Result<(), E>;
    fn visit_array_list_handle(
        &mut self,
        handle: ArrayListHandle,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_array_list_super(
        &mut self,
        view: ArrayListView,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_array_marker_handle(
        &mut self,
        handle: ArrayMarkerHandle,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_array_marker_super(
        &mut self,
        view: ArrayMarkerView,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_array_marker_opt_handle(
        &mut self,
        handle: ArrayMarkerOptHandle,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_array_marker_opt_super(
        &mut self,
        view: IntegerHandle,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_array_opt_handle(
        &mut self,
        handle: ArrayOptHandle,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_array_opt_super(&mut self, view: CommaHandle, tree: &Cst) -> Result<(), E>;
    fn visit_at_handle(&mut self, handle: AtHandle, tree: &Cst) -> Result<(), E>;
    fn visit_at_super(&mut self, view: AtView, tree: &Cst) -> Result<(), E>;
    fn visit_begin_handle(&mut self, handle: BeginHandle, tree: &Cst) -> Result<(), E>;
    fn visit_begin_super(&mut self, view: BeginView, tree: &Cst) -> Result<(), E>;
    fn visit_bind_handle(&mut self, handle: BindHandle, tree: &Cst) -> Result<(), E>;
    fn visit_bind_super(&mut self, view: BindView, tree: &Cst) -> Result<(), E>;
    fn visit_binding_handle(
        &mut self,
        handle: BindingHandle,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_binding_super(&mut self, view: BindingView, tree: &Cst) -> Result<(), E>;
    fn visit_binding_rhs_handle(
        &mut self,
        handle: BindingRhsHandle,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_binding_rhs_super(
        &mut self,
        view: BindingRhsView,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_boolean_handle(
        &mut self,
        handle: BooleanHandle,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_boolean_super(&mut self, view: BooleanView, tree: &Cst) -> Result<(), E>;
    fn visit_code_handle(&mut self, handle: CodeHandle, tree: &Cst) -> Result<(), E>;
    fn visit_code_super(&mut self, view: CodeView, tree: &Cst) -> Result<(), E>;
    fn visit_code_block_handle(
        &mut self,
        handle: CodeBlockHandle,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_code_block_super(
        &mut self,
        view: CodeBlockView,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_code_block_delimiter_handle(
        &mut self,
        handle: CodeBlockDelimiterHandle,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_code_block_delimiter_super(
        &mut self,
        view: CodeBlockDelimiterView,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_code_block_line_handle(
        &mut self,
        handle: CodeBlockLineHandle,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_code_block_line_super(
        &mut self,
        view: CodeBlockLineView,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_code_block_tail_common_handle(
        &mut self,
        handle: CodeBlockTailCommonHandle,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_code_block_tail_common_super(
        &mut self,
        view: CodeBlockTailCommonView,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_code_block_tail_common_list_handle(
        &mut self,
        handle: CodeBlockTailCommonListHandle,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_code_block_tail_common_list_super(
        &mut self,
        view: CodeBlockTailCommonListView,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_code_block_tail_common_opt_handle(
        &mut self,
        handle: CodeBlockTailCommonOptHandle,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_code_block_tail_common_opt_super(
        &mut self,
        view: WsHandle,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_comma_handle(&mut self, handle: CommaHandle, tree: &Cst) -> Result<(), E>;
    fn visit_comma_super(&mut self, view: CommaView, tree: &Cst) -> Result<(), E>;
    fn visit_continue_handle(
        &mut self,
        handle: ContinueHandle,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_continue_super(&mut self, view: ContinueView, tree: &Cst) -> Result<(), E>;
    fn visit_dot_handle(&mut self, handle: DotHandle, tree: &Cst) -> Result<(), E>;
    fn visit_dot_super(&mut self, view: DotView, tree: &Cst) -> Result<(), E>;
    fn visit_end_handle(&mut self, handle: EndHandle, tree: &Cst) -> Result<(), E>;
    fn visit_end_super(&mut self, view: EndView, tree: &Cst) -> Result<(), E>;
    fn visit_ext_handle(&mut self, handle: ExtHandle, tree: &Cst) -> Result<(), E>;
    fn visit_ext_super(&mut self, view: ExtView, tree: &Cst) -> Result<(), E>;
    fn visit_extension_name_space_handle(
        &mut self,
        handle: ExtensionNameSpaceHandle,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_extension_name_space_super(
        &mut self,
        view: ExtensionNameSpaceView,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_false_handle(&mut self, handle: FalseHandle, tree: &Cst) -> Result<(), E>;
    fn visit_false_super(&mut self, view: FalseView, tree: &Cst) -> Result<(), E>;
    fn visit_hole_handle(&mut self, handle: HoleHandle, tree: &Cst) -> Result<(), E>;
    fn visit_hole_super(&mut self, view: HoleView, tree: &Cst) -> Result<(), E>;
    fn visit_ident_handle(&mut self, handle: IdentHandle, tree: &Cst) -> Result<(), E>;
    fn visit_ident_super(&mut self, view: IdentView, tree: &Cst) -> Result<(), E>;
    fn visit_in_str_handle(&mut self, handle: InStrHandle, tree: &Cst) -> Result<(), E>;
    fn visit_in_str_super(&mut self, view: InStrView, tree: &Cst) -> Result<(), E>;
    fn visit_integer_handle(
        &mut self,
        handle: IntegerHandle,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_integer_super(&mut self, view: IntegerView, tree: &Cst) -> Result<(), E>;
    fn visit_key_handle(&mut self, handle: KeyHandle, tree: &Cst) -> Result<(), E>;
    fn visit_key_super(&mut self, view: KeyView, tree: &Cst) -> Result<(), E>;
    fn visit_key_base_handle(
        &mut self,
        handle: KeyBaseHandle,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_key_base_super(&mut self, view: KeyBaseView, tree: &Cst) -> Result<(), E>;
    fn visit_key_opt_handle(
        &mut self,
        handle: KeyOptHandle,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_key_opt_super(
        &mut self,
        view: ArrayMarkerHandle,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_keys_handle(&mut self, handle: KeysHandle, tree: &Cst) -> Result<(), E>;
    fn visit_keys_super(&mut self, view: KeysView, tree: &Cst) -> Result<(), E>;
    fn visit_keys_list_handle(
        &mut self,
        handle: KeysListHandle,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_keys_list_super(&mut self, view: KeysListView, tree: &Cst) -> Result<(), E>;
    fn visit_named_code_handle(
        &mut self,
        handle: NamedCodeHandle,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_named_code_super(
        &mut self,
        view: NamedCodeView,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_named_code_block_handle(
        &mut self,
        handle: NamedCodeBlockHandle,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_named_code_block_super(
        &mut self,
        view: NamedCodeBlockView,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_named_code_block_begin_handle(
        &mut self,
        handle: NamedCodeBlockBeginHandle,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_named_code_block_begin_super(
        &mut self,
        view: NamedCodeBlockBeginView,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_newline_handle(
        &mut self,
        handle: NewlineHandle,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_newline_super(&mut self, view: NewlineView, tree: &Cst) -> Result<(), E>;
    fn visit_null_handle(&mut self, handle: NullHandle, tree: &Cst) -> Result<(), E>;
    fn visit_null_super(&mut self, view: NullView, tree: &Cst) -> Result<(), E>;
    fn visit_object_handle(&mut self, handle: ObjectHandle, tree: &Cst) -> Result<(), E>;
    fn visit_object_super(&mut self, view: ObjectView, tree: &Cst) -> Result<(), E>;
    fn visit_object_list_handle(
        &mut self,
        handle: ObjectListHandle,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_object_list_super(
        &mut self,
        view: ObjectListView,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_object_opt_handle(
        &mut self,
        handle: ObjectOptHandle,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_object_opt_super(&mut self, view: CommaHandle, tree: &Cst) -> Result<(), E>;
    fn visit_quote_handle(&mut self, handle: QuoteHandle, tree: &Cst) -> Result<(), E>;
    fn visit_quote_super(&mut self, view: QuoteView, tree: &Cst) -> Result<(), E>;
    fn visit_section_handle(
        &mut self,
        handle: SectionHandle,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_section_super(&mut self, view: SectionView, tree: &Cst) -> Result<(), E>;
    fn visit_section_binding_handle(
        &mut self,
        handle: SectionBindingHandle,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_section_binding_super(
        &mut self,
        view: SectionBindingView,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_section_body_handle(
        &mut self,
        handle: SectionBodyHandle,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_section_body_super(
        &mut self,
        view: SectionBodyView,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_section_body_list_handle(
        &mut self,
        handle: SectionBodyListHandle,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_section_body_list_super(
        &mut self,
        view: SectionBodyListView,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_str_handle(&mut self, handle: StrHandle, tree: &Cst) -> Result<(), E>;
    fn visit_str_super(&mut self, view: StrView, tree: &Cst) -> Result<(), E>;
    fn visit_str_continues_handle(
        &mut self,
        handle: StrContinuesHandle,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_str_continues_super(
        &mut self,
        view: StrContinuesView,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_str_continues_list_handle(
        &mut self,
        handle: StrContinuesListHandle,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_str_continues_list_super(
        &mut self,
        view: StrContinuesListView,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_swon_handle(&mut self, handle: SwonHandle, tree: &Cst) -> Result<(), E>;
    fn visit_swon_super(&mut self, view: SwonView, tree: &Cst) -> Result<(), E>;
    fn visit_swon_bindings_handle(
        &mut self,
        handle: SwonBindingsHandle,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_swon_bindings_super(
        &mut self,
        view: SwonBindingsView,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_swon_sections_handle(
        &mut self,
        handle: SwonSectionsHandle,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_swon_sections_super(
        &mut self,
        view: SwonSectionsView,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_text_handle(&mut self, handle: TextHandle, tree: &Cst) -> Result<(), E>;
    fn visit_text_super(&mut self, view: TextView, tree: &Cst) -> Result<(), E>;
    fn visit_text_binding_handle(
        &mut self,
        handle: TextBindingHandle,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_text_binding_super(
        &mut self,
        view: TextBindingView,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_text_binding_opt_handle(
        &mut self,
        handle: TextBindingOptHandle,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_text_binding_opt_super(
        &mut self,
        view: WsHandle,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_text_start_handle(
        &mut self,
        handle: TextStartHandle,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_text_start_super(
        &mut self,
        view: TextStartView,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_true_handle(&mut self, handle: TrueHandle, tree: &Cst) -> Result<(), E>;
    fn visit_true_super(&mut self, view: TrueView, tree: &Cst) -> Result<(), E>;
    fn visit_typed_quote_handle(
        &mut self,
        handle: TypedQuoteHandle,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_typed_quote_super(
        &mut self,
        view: TypedQuoteView,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_typed_str_handle(
        &mut self,
        handle: TypedStrHandle,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_typed_str_super(&mut self, view: TypedStrView, tree: &Cst) -> Result<(), E>;
    fn visit_value_handle(&mut self, handle: ValueHandle, tree: &Cst) -> Result<(), E>;
    fn visit_value_super(&mut self, view: ValueView, tree: &Cst) -> Result<(), E>;
    fn visit_value_binding_handle(
        &mut self,
        handle: ValueBindingHandle,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_value_binding_super(
        &mut self,
        view: ValueBindingView,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_ws_handle(&mut self, handle: WsHandle, tree: &Cst) -> Result<(), E>;
    fn visit_ws_super(&mut self, view: WsView, tree: &Cst) -> Result<(), E>;
    fn visit_root_handle(&mut self, handle: RootHandle, tree: &Cst) -> Result<(), E>;
    fn visit_root_super(&mut self, view: RootView, tree: &Cst) -> Result<(), E>;
    fn visit_new_line_terminal_super(
        &mut self,
        terminal: NewLine,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_whitespace_terminal_super(
        &mut self,
        terminal: Whitespace,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_line_comment_terminal_super(
        &mut self,
        terminal: LineComment,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_block_comment_terminal_super(
        &mut self,
        terminal: BlockComment,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_integer_terminal_super(
        &mut self,
        terminal: Integer,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_true_terminal_super(
        &mut self,
        terminal: True,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_false_terminal_super(
        &mut self,
        terminal: False,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_null_terminal_super(
        &mut self,
        terminal: Null,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_hole_terminal_super(
        &mut self,
        terminal: Hole,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_quote_terminal_super(
        &mut self,
        terminal: Quote,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_typed_quote_terminal_super(
        &mut self,
        terminal: TypedQuote,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_in_str_terminal_super(
        &mut self,
        terminal: InStr,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_text_terminal_super(
        &mut self,
        terminal: Text,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_named_code_terminal_super(
        &mut self,
        terminal: NamedCode,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_code_terminal_super(
        &mut self,
        terminal: Code,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_newline_terminal_super(
        &mut self,
        terminal: Newline,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_ws_terminal_super(
        &mut self,
        terminal: Ws,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_at_terminal_super(
        &mut self,
        terminal: At,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_dollar_terminal_super(
        &mut self,
        terminal: Dollar,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_dot_terminal_super(
        &mut self,
        terminal: Dot,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_l_brace_terminal_super(
        &mut self,
        terminal: LBrace,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_r_brace_terminal_super(
        &mut self,
        terminal: RBrace,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_l_bracket_terminal_super(
        &mut self,
        terminal: LBracket,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_r_bracket_terminal_super(
        &mut self,
        terminal: RBracket,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_bind_terminal_super(
        &mut self,
        terminal: Bind,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_comma_terminal_super(
        &mut self,
        terminal: Comma,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_esc_terminal_super(
        &mut self,
        terminal: Esc,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_text_start_terminal_super(
        &mut self,
        terminal: TextStart,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_ident_terminal_super(
        &mut self,
        terminal: Ident,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_named_code_block_begin_terminal_super(
        &mut self,
        terminal: NamedCodeBlockBegin,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_code_block_delimiter_terminal_super(
        &mut self,
        terminal: CodeBlockDelimiter,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_code_block_line_terminal_super(
        &mut self,
        terminal: CodeBlockLine,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_non_terminal_super(
        &mut self,
        id: CstNodeId,
        kind: NonTerminalKind,
        data: NonTerminalData,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_non_terminal_close_super(
        &mut self,
        id: CstNodeId,
        kind: NonTerminalKind,
        data: NonTerminalData,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_terminal_super(
        &mut self,
        id: CstNodeId,
        kind: TerminalKind,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_any(&mut self, id: CstNodeId, node: CstNode, tree: &Cst) -> Result<(), E>;
    /// Recover from a construct error. This eagerly visits the children of the node.
    fn recover_error(
        &mut self,
        node_data: Option<CstNode>,
        id: CstNodeId,
        kind: NodeKind,
        tree: &Cst,
    ) -> Result<(), E>;
}
impl<V: CstVisitor> private::Sealed for V {}
impl<V: CstVisitor> CstHandleSuper<V::Error> for V {
    fn visit_array_handle(
        &mut self,
        handle: ArrayHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_array_begin_handle(
        &mut self,
        handle: ArrayBeginHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_array_end_handle(
        &mut self,
        handle: ArrayEndHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_array_list_handle(
        &mut self,
        handle: ArrayListHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_array_marker_handle(
        &mut self,
        handle: ArrayMarkerHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_array_marker_opt_handle(
        &mut self,
        handle: ArrayMarkerOptHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_array_opt_handle(
        &mut self,
        handle: ArrayOptHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_at_handle(&mut self, handle: AtHandle, tree: &Cst) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_begin_handle(
        &mut self,
        handle: BeginHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_bind_handle(
        &mut self,
        handle: BindHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_binding_handle(
        &mut self,
        handle: BindingHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_binding_rhs_handle(
        &mut self,
        handle: BindingRhsHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_boolean_handle(
        &mut self,
        handle: BooleanHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_code_handle(
        &mut self,
        handle: CodeHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_code_block_handle(
        &mut self,
        handle: CodeBlockHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_code_block_delimiter_handle(
        &mut self,
        handle: CodeBlockDelimiterHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (
                    visit.visit_code_block_delimiter(handle, view, tree),
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_code_block_line_handle(
        &mut self,
        handle: CodeBlockLineHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (
                    visit.visit_code_block_line(handle, view, tree),
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_code_block_tail_common_handle(
        &mut self,
        handle: CodeBlockTailCommonHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (
                    visit.visit_code_block_tail_common(handle, view, tree),
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_code_block_tail_common_list_handle(
        &mut self,
        handle: CodeBlockTailCommonListHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (
                    if let Some(view) = view {
                        visit.visit_code_block_tail_common_list(handle, view, tree)
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_code_block_tail_common_opt_handle(
        &mut self,
        handle: CodeBlockTailCommonOptHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (
                    if let Some(view) = view {
                        visit.visit_code_block_tail_common_opt(handle, view, tree)
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_comma_handle(
        &mut self,
        handle: CommaHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_continue_handle(
        &mut self,
        handle: ContinueHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_dot_handle(
        &mut self,
        handle: DotHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_end_handle(
        &mut self,
        handle: EndHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_ext_handle(
        &mut self,
        handle: ExtHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_extension_name_space_handle(
        &mut self,
        handle: ExtensionNameSpaceHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_false_handle(
        &mut self,
        handle: FalseHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_hole_handle(
        &mut self,
        handle: HoleHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_ident_handle(
        &mut self,
        handle: IdentHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_in_str_handle(
        &mut self,
        handle: InStrHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_integer_handle(
        &mut self,
        handle: IntegerHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_key_handle(
        &mut self,
        handle: KeyHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_key_base_handle(
        &mut self,
        handle: KeyBaseHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_key_opt_handle(
        &mut self,
        handle: KeyOptHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_keys_handle(
        &mut self,
        handle: KeysHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_keys_list_handle(
        &mut self,
        handle: KeysListHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_named_code_handle(
        &mut self,
        handle: NamedCodeHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_named_code_block_handle(
        &mut self,
        handle: NamedCodeBlockHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (
                    visit.visit_named_code_block(handle, view, tree),
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_named_code_block_begin_handle(
        &mut self,
        handle: NamedCodeBlockBeginHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
        let result = match handle
            .get_view_with_visit(
                tree,
                |view, visit: &mut Self| (
                    visit.visit_named_code_block_begin(handle, view, tree),
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_newline_handle(
        &mut self,
        handle: NewlineHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_null_handle(
        &mut self,
        handle: NullHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_object_handle(
        &mut self,
        handle: ObjectHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_object_list_handle(
        &mut self,
        handle: ObjectListHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_object_opt_handle(
        &mut self,
        handle: ObjectOptHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_quote_handle(
        &mut self,
        handle: QuoteHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_section_handle(
        &mut self,
        handle: SectionHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_section_binding_handle(
        &mut self,
        handle: SectionBindingHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_section_body_handle(
        &mut self,
        handle: SectionBodyHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_section_body_list_handle(
        &mut self,
        handle: SectionBodyListHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_str_handle(
        &mut self,
        handle: StrHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_str_continues_handle(
        &mut self,
        handle: StrContinuesHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_str_continues_list_handle(
        &mut self,
        handle: StrContinuesListHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_swon_handle(
        &mut self,
        handle: SwonHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_swon_bindings_handle(
        &mut self,
        handle: SwonBindingsHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_swon_sections_handle(
        &mut self,
        handle: SwonSectionsHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_text_handle(
        &mut self,
        handle: TextHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_text_binding_handle(
        &mut self,
        handle: TextBindingHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_text_binding_opt_handle(
        &mut self,
        handle: TextBindingOptHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_text_start_handle(
        &mut self,
        handle: TextStartHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_true_handle(
        &mut self,
        handle: TrueHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_typed_quote_handle(
        &mut self,
        handle: TypedQuoteHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_typed_str_handle(
        &mut self,
        handle: TypedStrHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_value_handle(
        &mut self,
        handle: ValueHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_value_binding_handle(
        &mut self,
        handle: ValueBindingHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_ws_handle(&mut self, handle: WsHandle, tree: &Cst) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_root_handle(
        &mut self,
        handle: RootHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let Some(node_data) = tree.node_data(handle.node_id()) else {
            return self
                .then_construct_error(
                    None,
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: handle.node_id(),
                    },
                    tree,
                );
        };
        let (kind, nt_data) = match node_data
            .expected_non_terminal_or_error(handle.node_id(), handle.kind())
        {
            Ok((kind, nt_data)) => (kind, nt_data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        handle.node_id(),
                        NodeKind::NonTerminal(handle.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_non_terminal(handle.node_id(), kind, nt_data, tree)?;
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
                    Some(node_data),
                    handle.node_id(),
                    NodeKind::NonTerminal(handle.kind()),
                    e,
                    tree,
                )
            }
        };
        self.visit_non_terminal_close(handle.node_id(), kind, nt_data, tree)?;
        result
    }
    fn visit_array_super(
        &mut self,
        view_param: ArrayView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let ArrayView { array_begin, array_list, array_end } = view_param;
        self.visit_array_begin_handle(array_begin, tree)?;
        self.visit_array_list_handle(array_list, tree)?;
        self.visit_array_end_handle(array_end, tree)?;
        Ok(())
    }
    fn visit_array_begin_super(
        &mut self,
        view_param: ArrayBeginView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let ArrayBeginView { l_bracket } = view_param;
        let Some(node_data) = tree.node_data(l_bracket.0) else {
            return self
                .then_construct_error(
                    None,
                    l_bracket.0,
                    NodeKind::Terminal(l_bracket.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: l_bracket.0,
                    },
                    tree,
                );
        };
        let (_kind, data) = match node_data
            .expected_terminal_or_error(l_bracket.0, l_bracket.kind())
        {
            Ok((_kind, data)) => (data, data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
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
        view_param: ArrayEndView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let ArrayEndView { r_bracket } = view_param;
        let Some(node_data) = tree.node_data(r_bracket.0) else {
            return self
                .then_construct_error(
                    None,
                    r_bracket.0,
                    NodeKind::Terminal(r_bracket.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: r_bracket.0,
                    },
                    tree,
                );
        };
        let (_kind, data) = match node_data
            .expected_terminal_or_error(r_bracket.0, r_bracket.kind())
        {
            Ok((_kind, data)) => (data, data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
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
        view_param: ArrayListView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let ArrayListView { value, array_opt, array_list } = view_param;
        self.visit_value_handle(value, tree)?;
        self.visit_array_opt_handle(array_opt, tree)?;
        self.visit_array_list_handle(array_list, tree)?;
        Ok(())
    }
    fn visit_array_marker_super(
        &mut self,
        view_param: ArrayMarkerView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let ArrayMarkerView { array_begin, array_marker_opt, array_end } = view_param;
        self.visit_array_begin_handle(array_begin, tree)?;
        self.visit_array_marker_opt_handle(array_marker_opt, tree)?;
        self.visit_array_end_handle(array_end, tree)?;
        Ok(())
    }
    fn visit_array_marker_opt_super(
        &mut self,
        view_param: IntegerHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        self.visit_integer_handle(view_param, tree)?;
        Ok(())
    }
    fn visit_array_opt_super(
        &mut self,
        view_param: CommaHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        self.visit_comma_handle(view_param, tree)?;
        Ok(())
    }
    fn visit_at_super(
        &mut self,
        view_param: AtView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let AtView { at } = view_param;
        let Some(node_data) = tree.node_data(at.0) else {
            return self
                .then_construct_error(
                    None,
                    at.0,
                    NodeKind::Terminal(at.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: at.0,
                    },
                    tree,
                );
        };
        let (_kind, data) = match node_data.expected_terminal_or_error(at.0, at.kind()) {
            Ok((_kind, data)) => (data, data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
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
        view_param: BeginView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let BeginView { l_brace } = view_param;
        let Some(node_data) = tree.node_data(l_brace.0) else {
            return self
                .then_construct_error(
                    None,
                    l_brace.0,
                    NodeKind::Terminal(l_brace.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: l_brace.0,
                    },
                    tree,
                );
        };
        let (_kind, data) = match node_data
            .expected_terminal_or_error(l_brace.0, l_brace.kind())
        {
            Ok((_kind, data)) => (data, data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
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
        view_param: BindView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let BindView { bind } = view_param;
        let Some(node_data) = tree.node_data(bind.0) else {
            return self
                .then_construct_error(
                    None,
                    bind.0,
                    NodeKind::Terminal(bind.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: bind.0,
                    },
                    tree,
                );
        };
        let (_kind, data) = match node_data
            .expected_terminal_or_error(bind.0, bind.kind())
        {
            Ok((_kind, data)) => (data, data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
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
        view_param: BindingView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let BindingView { keys, binding_rhs } = view_param;
        self.visit_keys_handle(keys, tree)?;
        self.visit_binding_rhs_handle(binding_rhs, tree)?;
        Ok(())
    }
    fn visit_binding_rhs_super(
        &mut self,
        view_param: BindingRhsView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
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
        view_param: BooleanView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
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
        view_param: CodeView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let CodeView { code } = view_param;
        let Some(node_data) = tree.node_data(code.0) else {
            return self
                .then_construct_error(
                    None,
                    code.0,
                    NodeKind::Terminal(code.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: code.0,
                    },
                    tree,
                );
        };
        let (_kind, data) = match node_data
            .expected_terminal_or_error(code.0, code.kind())
        {
            Ok((_kind, data)) => (data, data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
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
        view_param: CodeBlockView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let CodeBlockView { code_block_delimiter, code_block_tail_common } = view_param;
        self.visit_code_block_delimiter_handle(code_block_delimiter, tree)?;
        self.visit_code_block_tail_common_handle(code_block_tail_common, tree)?;
        Ok(())
    }
    fn visit_code_block_delimiter_super(
        &mut self,
        view_param: CodeBlockDelimiterView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let CodeBlockDelimiterView { code_block_delimiter } = view_param;
        let Some(node_data) = tree.node_data(code_block_delimiter.0) else {
            return self
                .then_construct_error(
                    None,
                    code_block_delimiter.0,
                    NodeKind::Terminal(code_block_delimiter.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: code_block_delimiter.0,
                    },
                    tree,
                );
        };
        let (_kind, data) = match node_data
            .expected_terminal_or_error(
                code_block_delimiter.0,
                code_block_delimiter.kind(),
            )
        {
            Ok((_kind, data)) => (data, data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        code_block_delimiter.0,
                        NodeKind::Terminal(code_block_delimiter.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_code_block_delimiter_terminal(code_block_delimiter, data, tree)?;
        Ok(())
    }
    fn visit_code_block_line_super(
        &mut self,
        view_param: CodeBlockLineView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let CodeBlockLineView { code_block_line } = view_param;
        let Some(node_data) = tree.node_data(code_block_line.0) else {
            return self
                .then_construct_error(
                    None,
                    code_block_line.0,
                    NodeKind::Terminal(code_block_line.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: code_block_line.0,
                    },
                    tree,
                );
        };
        let (_kind, data) = match node_data
            .expected_terminal_or_error(code_block_line.0, code_block_line.kind())
        {
            Ok((_kind, data)) => (data, data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        code_block_line.0,
                        NodeKind::Terminal(code_block_line.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_code_block_line_terminal(code_block_line, data, tree)?;
        Ok(())
    }
    fn visit_code_block_tail_common_super(
        &mut self,
        view_param: CodeBlockTailCommonView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let CodeBlockTailCommonView {
            newline,
            code_block_tail_common_list,
            code_block_tail_common_opt,
            code_block_delimiter,
        } = view_param;
        self.visit_newline_handle(newline, tree)?;
        self.visit_code_block_tail_common_list_handle(
            code_block_tail_common_list,
            tree,
        )?;
        self.visit_code_block_tail_common_opt_handle(code_block_tail_common_opt, tree)?;
        self.visit_code_block_delimiter_handle(code_block_delimiter, tree)?;
        Ok(())
    }
    fn visit_code_block_tail_common_list_super(
        &mut self,
        view_param: CodeBlockTailCommonListView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let CodeBlockTailCommonListView {
            code_block_line,
            newline,
            code_block_tail_common_list,
        } = view_param;
        self.visit_code_block_line_handle(code_block_line, tree)?;
        self.visit_newline_handle(newline, tree)?;
        self.visit_code_block_tail_common_list_handle(
            code_block_tail_common_list,
            tree,
        )?;
        Ok(())
    }
    fn visit_code_block_tail_common_opt_super(
        &mut self,
        view_param: WsHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        self.visit_ws_handle(view_param, tree)?;
        Ok(())
    }
    fn visit_comma_super(
        &mut self,
        view_param: CommaView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let CommaView { comma } = view_param;
        let Some(node_data) = tree.node_data(comma.0) else {
            return self
                .then_construct_error(
                    None,
                    comma.0,
                    NodeKind::Terminal(comma.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: comma.0,
                    },
                    tree,
                );
        };
        let (_kind, data) = match node_data
            .expected_terminal_or_error(comma.0, comma.kind())
        {
            Ok((_kind, data)) => (data, data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
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
        view_param: ContinueView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let ContinueView { esc } = view_param;
        let Some(node_data) = tree.node_data(esc.0) else {
            return self
                .then_construct_error(
                    None,
                    esc.0,
                    NodeKind::Terminal(esc.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: esc.0,
                    },
                    tree,
                );
        };
        let (_kind, data) = match node_data.expected_terminal_or_error(esc.0, esc.kind())
        {
            Ok((_kind, data)) => (data, data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
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
        view_param: DotView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let DotView { dot } = view_param;
        let Some(node_data) = tree.node_data(dot.0) else {
            return self
                .then_construct_error(
                    None,
                    dot.0,
                    NodeKind::Terminal(dot.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: dot.0,
                    },
                    tree,
                );
        };
        let (_kind, data) = match node_data.expected_terminal_or_error(dot.0, dot.kind())
        {
            Ok((_kind, data)) => (data, data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
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
        view_param: EndView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let EndView { r_brace } = view_param;
        let Some(node_data) = tree.node_data(r_brace.0) else {
            return self
                .then_construct_error(
                    None,
                    r_brace.0,
                    NodeKind::Terminal(r_brace.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: r_brace.0,
                    },
                    tree,
                );
        };
        let (_kind, data) = match node_data
            .expected_terminal_or_error(r_brace.0, r_brace.kind())
        {
            Ok((_kind, data)) => (data, data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
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
        view_param: ExtView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let ExtView { dollar } = view_param;
        let Some(node_data) = tree.node_data(dollar.0) else {
            return self
                .then_construct_error(
                    None,
                    dollar.0,
                    NodeKind::Terminal(dollar.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: dollar.0,
                    },
                    tree,
                );
        };
        let (_kind, data) = match node_data
            .expected_terminal_or_error(dollar.0, dollar.kind())
        {
            Ok((_kind, data)) => (data, data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
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
        view_param: ExtensionNameSpaceView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let ExtensionNameSpaceView { ext, ident } = view_param;
        self.visit_ext_handle(ext, tree)?;
        self.visit_ident_handle(ident, tree)?;
        Ok(())
    }
    fn visit_false_super(
        &mut self,
        view_param: FalseView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let FalseView { r#false } = view_param;
        let Some(node_data) = tree.node_data(r#false.0) else {
            return self
                .then_construct_error(
                    None,
                    r#false.0,
                    NodeKind::Terminal(r#false.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: r#false.0,
                    },
                    tree,
                );
        };
        let (_kind, data) = match node_data
            .expected_terminal_or_error(r#false.0, r#false.kind())
        {
            Ok((_kind, data)) => (data, data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
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
        view_param: HoleView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let HoleView { hole } = view_param;
        let Some(node_data) = tree.node_data(hole.0) else {
            return self
                .then_construct_error(
                    None,
                    hole.0,
                    NodeKind::Terminal(hole.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: hole.0,
                    },
                    tree,
                );
        };
        let (_kind, data) = match node_data
            .expected_terminal_or_error(hole.0, hole.kind())
        {
            Ok((_kind, data)) => (data, data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
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
        view_param: IdentView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let IdentView { ident } = view_param;
        let Some(node_data) = tree.node_data(ident.0) else {
            return self
                .then_construct_error(
                    None,
                    ident.0,
                    NodeKind::Terminal(ident.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: ident.0,
                    },
                    tree,
                );
        };
        let (_kind, data) = match node_data
            .expected_terminal_or_error(ident.0, ident.kind())
        {
            Ok((_kind, data)) => (data, data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
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
        view_param: InStrView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let InStrView { in_str } = view_param;
        let Some(node_data) = tree.node_data(in_str.0) else {
            return self
                .then_construct_error(
                    None,
                    in_str.0,
                    NodeKind::Terminal(in_str.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: in_str.0,
                    },
                    tree,
                );
        };
        let (_kind, data) = match node_data
            .expected_terminal_or_error(in_str.0, in_str.kind())
        {
            Ok((_kind, data)) => (data, data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
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
        view_param: IntegerView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let IntegerView { integer } = view_param;
        let Some(node_data) = tree.node_data(integer.0) else {
            return self
                .then_construct_error(
                    None,
                    integer.0,
                    NodeKind::Terminal(integer.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: integer.0,
                    },
                    tree,
                );
        };
        let (_kind, data) = match node_data
            .expected_terminal_or_error(integer.0, integer.kind())
        {
            Ok((_kind, data)) => (data, data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
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
        view_param: KeyView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let KeyView { key_base, key_opt } = view_param;
        self.visit_key_base_handle(key_base, tree)?;
        self.visit_key_opt_handle(key_opt, tree)?;
        Ok(())
    }
    fn visit_key_base_super(
        &mut self,
        view_param: KeyBaseView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
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
        view_param: ArrayMarkerHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        self.visit_array_marker_handle(view_param, tree)?;
        Ok(())
    }
    fn visit_keys_super(
        &mut self,
        view_param: KeysView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let KeysView { key, keys_list } = view_param;
        self.visit_key_handle(key, tree)?;
        self.visit_keys_list_handle(keys_list, tree)?;
        Ok(())
    }
    fn visit_keys_list_super(
        &mut self,
        view_param: KeysListView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let KeysListView { dot, key, keys_list } = view_param;
        self.visit_dot_handle(dot, tree)?;
        self.visit_key_handle(key, tree)?;
        self.visit_keys_list_handle(keys_list, tree)?;
        Ok(())
    }
    fn visit_named_code_super(
        &mut self,
        view_param: NamedCodeView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let NamedCodeView { named_code } = view_param;
        let Some(node_data) = tree.node_data(named_code.0) else {
            return self
                .then_construct_error(
                    None,
                    named_code.0,
                    NodeKind::Terminal(named_code.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: named_code.0,
                    },
                    tree,
                );
        };
        let (_kind, data) = match node_data
            .expected_terminal_or_error(named_code.0, named_code.kind())
        {
            Ok((_kind, data)) => (data, data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
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
    fn visit_named_code_block_super(
        &mut self,
        view_param: NamedCodeBlockView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let NamedCodeBlockView { named_code_block_begin, code_block_tail_common } = view_param;
        self.visit_named_code_block_begin_handle(named_code_block_begin, tree)?;
        self.visit_code_block_tail_common_handle(code_block_tail_common, tree)?;
        Ok(())
    }
    fn visit_named_code_block_begin_super(
        &mut self,
        view_param: NamedCodeBlockBeginView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let NamedCodeBlockBeginView { named_code_block_begin } = view_param;
        let Some(node_data) = tree.node_data(named_code_block_begin.0) else {
            return self
                .then_construct_error(
                    None,
                    named_code_block_begin.0,
                    NodeKind::Terminal(named_code_block_begin.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: named_code_block_begin.0,
                    },
                    tree,
                );
        };
        let (_kind, data) = match node_data
            .expected_terminal_or_error(
                named_code_block_begin.0,
                named_code_block_begin.kind(),
            )
        {
            Ok((_kind, data)) => (data, data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
                        named_code_block_begin.0,
                        NodeKind::Terminal(named_code_block_begin.kind()),
                        error,
                        tree,
                    );
            }
        };
        self.visit_named_code_block_begin_terminal(named_code_block_begin, data, tree)?;
        Ok(())
    }
    fn visit_newline_super(
        &mut self,
        view_param: NewlineView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let NewlineView { newline } = view_param;
        let Some(node_data) = tree.node_data(newline.0) else {
            return self
                .then_construct_error(
                    None,
                    newline.0,
                    NodeKind::Terminal(newline.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: newline.0,
                    },
                    tree,
                );
        };
        let (_kind, data) = match node_data
            .expected_terminal_or_error(newline.0, newline.kind())
        {
            Ok((_kind, data)) => (data, data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
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
        view_param: NullView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let NullView { null } = view_param;
        let Some(node_data) = tree.node_data(null.0) else {
            return self
                .then_construct_error(
                    None,
                    null.0,
                    NodeKind::Terminal(null.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: null.0,
                    },
                    tree,
                );
        };
        let (_kind, data) = match node_data
            .expected_terminal_or_error(null.0, null.kind())
        {
            Ok((_kind, data)) => (data, data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
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
        view_param: ObjectView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let ObjectView { begin, object_list, end } = view_param;
        self.visit_begin_handle(begin, tree)?;
        self.visit_object_list_handle(object_list, tree)?;
        self.visit_end_handle(end, tree)?;
        Ok(())
    }
    fn visit_object_list_super(
        &mut self,
        view_param: ObjectListView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
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
        view_param: CommaHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        self.visit_comma_handle(view_param, tree)?;
        Ok(())
    }
    fn visit_quote_super(
        &mut self,
        view_param: QuoteView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let QuoteView { quote } = view_param;
        let Some(node_data) = tree.node_data(quote.0) else {
            return self
                .then_construct_error(
                    None,
                    quote.0,
                    NodeKind::Terminal(quote.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: quote.0,
                    },
                    tree,
                );
        };
        let (_kind, data) = match node_data
            .expected_terminal_or_error(quote.0, quote.kind())
        {
            Ok((_kind, data)) => (data, data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
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
        view_param: SectionView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let SectionView { at, keys, section_body } = view_param;
        self.visit_at_handle(at, tree)?;
        self.visit_keys_handle(keys, tree)?;
        self.visit_section_body_handle(section_body, tree)?;
        Ok(())
    }
    fn visit_section_binding_super(
        &mut self,
        view_param: SectionBindingView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let SectionBindingView { begin, swon, end } = view_param;
        self.visit_begin_handle(begin, tree)?;
        self.visit_swon_handle(swon, tree)?;
        self.visit_end_handle(end, tree)?;
        Ok(())
    }
    fn visit_section_body_super(
        &mut self,
        view_param: SectionBodyView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
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
        view_param: SectionBodyListView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let SectionBodyListView { binding, section_body_list } = view_param;
        self.visit_binding_handle(binding, tree)?;
        self.visit_section_body_list_handle(section_body_list, tree)?;
        Ok(())
    }
    fn visit_str_super(
        &mut self,
        view_param: StrView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let StrView { quote, in_str, quote2 } = view_param;
        self.visit_quote_handle(quote, tree)?;
        self.visit_in_str_handle(in_str, tree)?;
        self.visit_quote_handle(quote2, tree)?;
        Ok(())
    }
    fn visit_str_continues_super(
        &mut self,
        view_param: StrContinuesView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let StrContinuesView { str, str_continues_list } = view_param;
        self.visit_str_handle(str, tree)?;
        self.visit_str_continues_list_handle(str_continues_list, tree)?;
        Ok(())
    }
    fn visit_str_continues_list_super(
        &mut self,
        view_param: StrContinuesListView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let StrContinuesListView { r#continue, str, str_continues_list } = view_param;
        self.visit_continue_handle(r#continue, tree)?;
        self.visit_str_handle(str, tree)?;
        self.visit_str_continues_list_handle(str_continues_list, tree)?;
        Ok(())
    }
    fn visit_swon_super(
        &mut self,
        view_param: SwonView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let SwonView { swon_bindings, swon_sections } = view_param;
        self.visit_swon_bindings_handle(swon_bindings, tree)?;
        self.visit_swon_sections_handle(swon_sections, tree)?;
        Ok(())
    }
    fn visit_swon_bindings_super(
        &mut self,
        view_param: SwonBindingsView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let SwonBindingsView { binding, swon_bindings } = view_param;
        self.visit_binding_handle(binding, tree)?;
        self.visit_swon_bindings_handle(swon_bindings, tree)?;
        Ok(())
    }
    fn visit_swon_sections_super(
        &mut self,
        view_param: SwonSectionsView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let SwonSectionsView { section, swon_sections } = view_param;
        self.visit_section_handle(section, tree)?;
        self.visit_swon_sections_handle(swon_sections, tree)?;
        Ok(())
    }
    fn visit_text_super(
        &mut self,
        view_param: TextView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let TextView { text } = view_param;
        let Some(node_data) = tree.node_data(text.0) else {
            return self
                .then_construct_error(
                    None,
                    text.0,
                    NodeKind::Terminal(text.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: text.0,
                    },
                    tree,
                );
        };
        let (_kind, data) = match node_data
            .expected_terminal_or_error(text.0, text.kind())
        {
            Ok((_kind, data)) => (data, data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
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
        view_param: TextBindingView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let TextBindingView { text_start, text_binding_opt, text, newline } = view_param;
        self.visit_text_start_handle(text_start, tree)?;
        self.visit_text_binding_opt_handle(text_binding_opt, tree)?;
        self.visit_text_handle(text, tree)?;
        self.visit_newline_handle(newline, tree)?;
        Ok(())
    }
    fn visit_text_binding_opt_super(
        &mut self,
        view_param: WsHandle,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        self.visit_ws_handle(view_param, tree)?;
        Ok(())
    }
    fn visit_text_start_super(
        &mut self,
        view_param: TextStartView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let TextStartView { text_start } = view_param;
        let Some(node_data) = tree.node_data(text_start.0) else {
            return self
                .then_construct_error(
                    None,
                    text_start.0,
                    NodeKind::Terminal(text_start.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: text_start.0,
                    },
                    tree,
                );
        };
        let (_kind, data) = match node_data
            .expected_terminal_or_error(text_start.0, text_start.kind())
        {
            Ok((_kind, data)) => (data, data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
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
        view_param: TrueView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let TrueView { r#true } = view_param;
        let Some(node_data) = tree.node_data(r#true.0) else {
            return self
                .then_construct_error(
                    None,
                    r#true.0,
                    NodeKind::Terminal(r#true.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: r#true.0,
                    },
                    tree,
                );
        };
        let (_kind, data) = match node_data
            .expected_terminal_or_error(r#true.0, r#true.kind())
        {
            Ok((_kind, data)) => (data, data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
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
        view_param: TypedQuoteView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let TypedQuoteView { typed_quote } = view_param;
        let Some(node_data) = tree.node_data(typed_quote.0) else {
            return self
                .then_construct_error(
                    None,
                    typed_quote.0,
                    NodeKind::Terminal(typed_quote.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: typed_quote.0,
                    },
                    tree,
                );
        };
        let (_kind, data) = match node_data
            .expected_terminal_or_error(typed_quote.0, typed_quote.kind())
        {
            Ok((_kind, data)) => (data, data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
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
        view_param: TypedStrView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let TypedStrView { typed_quote, in_str, quote } = view_param;
        self.visit_typed_quote_handle(typed_quote, tree)?;
        self.visit_in_str_handle(in_str, tree)?;
        self.visit_quote_handle(quote, tree)?;
        Ok(())
    }
    fn visit_value_super(
        &mut self,
        view_param: ValueView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
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
            ValueView::NamedCodeBlock(item) => {
                self.visit_named_code_block_handle(item, tree)?;
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
        view_param: ValueBindingView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let ValueBindingView { bind, value } = view_param;
        self.visit_bind_handle(bind, tree)?;
        self.visit_value_handle(value, tree)?;
        Ok(())
    }
    fn visit_ws_super(
        &mut self,
        view_param: WsView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let WsView { ws } = view_param;
        let Some(node_data) = tree.node_data(ws.0) else {
            return self
                .then_construct_error(
                    None,
                    ws.0,
                    NodeKind::Terminal(ws.kind()),
                    CstConstructError::NodeIdNotFound {
                        node: ws.0,
                    },
                    tree,
                );
        };
        let (_kind, data) = match node_data.expected_terminal_or_error(ws.0, ws.kind()) {
            Ok((_kind, data)) => (data, data),
            Err(error) => {
                return self
                    .then_construct_error(
                        Some(node_data),
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
        view_param: RootView,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        let RootView { swon } = view_param;
        self.visit_swon_handle(swon, tree)?;
        Ok(())
    }
    fn visit_new_line_terminal_super(
        &mut self,
        terminal: NewLine,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_whitespace_terminal_super(
        &mut self,
        terminal: Whitespace,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_line_comment_terminal_super(
        &mut self,
        terminal: LineComment,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_block_comment_terminal_super(
        &mut self,
        terminal: BlockComment,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_integer_terminal_super(
        &mut self,
        terminal: Integer,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_true_terminal_super(
        &mut self,
        terminal: True,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_false_terminal_super(
        &mut self,
        terminal: False,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_null_terminal_super(
        &mut self,
        terminal: Null,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_hole_terminal_super(
        &mut self,
        terminal: Hole,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_quote_terminal_super(
        &mut self,
        terminal: Quote,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_typed_quote_terminal_super(
        &mut self,
        terminal: TypedQuote,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_in_str_terminal_super(
        &mut self,
        terminal: InStr,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_text_terminal_super(
        &mut self,
        terminal: Text,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_named_code_terminal_super(
        &mut self,
        terminal: NamedCode,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_code_terminal_super(
        &mut self,
        terminal: Code,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_newline_terminal_super(
        &mut self,
        terminal: Newline,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_ws_terminal_super(
        &mut self,
        terminal: Ws,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_at_terminal_super(
        &mut self,
        terminal: At,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_dollar_terminal_super(
        &mut self,
        terminal: Dollar,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_dot_terminal_super(
        &mut self,
        terminal: Dot,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_l_brace_terminal_super(
        &mut self,
        terminal: LBrace,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_r_brace_terminal_super(
        &mut self,
        terminal: RBrace,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_l_bracket_terminal_super(
        &mut self,
        terminal: LBracket,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_r_bracket_terminal_super(
        &mut self,
        terminal: RBracket,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_bind_terminal_super(
        &mut self,
        terminal: Bind,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_comma_terminal_super(
        &mut self,
        terminal: Comma,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_esc_terminal_super(
        &mut self,
        terminal: Esc,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_text_start_terminal_super(
        &mut self,
        terminal: TextStart,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_ident_terminal_super(
        &mut self,
        terminal: Ident,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_named_code_block_begin_terminal_super(
        &mut self,
        terminal: NamedCodeBlockBegin,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_code_block_delimiter_terminal_super(
        &mut self,
        terminal: CodeBlockDelimiter,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_code_block_line_terminal_super(
        &mut self,
        terminal: CodeBlockLine,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        self.visit_terminal(terminal.0, terminal.kind(), data, tree)?;
        Ok(())
    }
    fn visit_non_terminal_super(
        &mut self,
        _id: CstNodeId,
        _kind: NonTerminalKind,
        _data: NonTerminalData,
        _tree: &Cst,
    ) -> Result<(), V::Error> {
        Ok(())
    }
    fn visit_non_terminal_close_super(
        &mut self,
        _id: CstNodeId,
        _kind: NonTerminalKind,
        _data: NonTerminalData,
        _tree: &Cst,
    ) -> Result<(), V::Error> {
        Ok(())
    }
    fn visit_terminal_super(
        &mut self,
        _id: CstNodeId,
        _kind: TerminalKind,
        _data: TerminalData,
        _tree: &Cst,
    ) -> Result<(), V::Error> {
        Ok(())
    }
    fn recover_error(
        &mut self,
        node_data: Option<CstNode>,
        id: CstNodeId,
        kind: NodeKind,
        tree: &Cst,
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
        tree: &Cst,
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
                    NonTerminalKind::CodeBlockDelimiter => {
                        let handle = CodeBlockDelimiterHandle(id);
                        self.visit_code_block_delimiter_handle(handle, tree)?;
                    }
                    NonTerminalKind::CodeBlockLine => {
                        let handle = CodeBlockLineHandle(id);
                        self.visit_code_block_line_handle(handle, tree)?;
                    }
                    NonTerminalKind::CodeBlockTailCommon => {
                        let handle = CodeBlockTailCommonHandle(id);
                        self.visit_code_block_tail_common_handle(handle, tree)?;
                    }
                    NonTerminalKind::CodeBlockTailCommonList => {
                        let handle = CodeBlockTailCommonListHandle(id);
                        self.visit_code_block_tail_common_list_handle(handle, tree)?;
                    }
                    NonTerminalKind::CodeBlockTailCommonOpt => {
                        let handle = CodeBlockTailCommonOptHandle(id);
                        self.visit_code_block_tail_common_opt_handle(handle, tree)?;
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
                    NonTerminalKind::NamedCodeBlock => {
                        let handle = NamedCodeBlockHandle(id);
                        self.visit_named_code_block_handle(handle, tree)?;
                    }
                    NonTerminalKind::NamedCodeBlockBegin => {
                        let handle = NamedCodeBlockBeginHandle(id);
                        self.visit_named_code_block_begin_handle(handle, tree)?;
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
                    TerminalKind::NamedCodeBlockBegin => {
                        let terminal = NamedCodeBlockBegin(id);
                        self.visit_named_code_block_begin_terminal(
                            terminal,
                            data,
                            tree,
                        )?;
                    }
                    TerminalKind::CodeBlockDelimiter => {
                        let terminal = CodeBlockDelimiter(id);
                        self.visit_code_block_delimiter_terminal(terminal, data, tree)?;
                    }
                    TerminalKind::CodeBlockLine => {
                        let terminal = CodeBlockLine(id);
                        self.visit_code_block_line_terminal(terminal, data, tree)?;
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
pub trait BuiltinTerminalVisitor<E> {
    fn visit_builtin_new_line_terminal(
        &mut self,
        terminal: NewLine,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_builtin_whitespace_terminal(
        &mut self,
        terminal: Whitespace,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_builtin_line_comment_terminal(
        &mut self,
        terminal: LineComment,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), E>;
    fn visit_builtin_block_comment_terminal(
        &mut self,
        terminal: BlockComment,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), E>;
}
impl<V: CstVisitor> BuiltinTerminalVisitor<V::Error> for V {
    fn visit_builtin_new_line_terminal(
        &mut self,
        terminal: NewLine,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        self.visit_new_line_terminal(terminal, data, tree)
    }
    fn visit_builtin_whitespace_terminal(
        &mut self,
        terminal: Whitespace,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        self.visit_whitespace_terminal(terminal, data, tree)
    }
    fn visit_builtin_line_comment_terminal(
        &mut self,
        terminal: LineComment,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        self.visit_line_comment_terminal(terminal, data, tree)
    }
    fn visit_builtin_block_comment_terminal(
        &mut self,
        terminal: BlockComment,
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), V::Error> {
        self.visit_block_comment_terminal(terminal, data, tree)
    }
}
