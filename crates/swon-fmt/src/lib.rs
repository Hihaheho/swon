use std::convert::Infallible;

use swon_tree::prelude::*;

pub fn fmt(input: &str, cst: &mut Cst) {
    let mut formatter = Formatter::new();
    let Ok(_) = formatter.visit_root_handle(cst.root_handle(), cst);
}

pub enum FmtError {
    InvalidWhitespace { id: CstNodeId },
}

pub struct Formatter {
    desired_indent: usize,
    new_line_count: usize,
    current_indent: usize,
    errors: Vec<(FmtError, CstCommands)>,
}

impl Formatter {
    fn new() -> Self {
        Self {
            desired_indent: 0,
            new_line_count: 0,
            current_indent: 0,
            errors: vec![],
        }
    }
}

impl Default for Formatter {
    fn default() -> Self {
        Self::new()
    }
}

impl<F: CstFacade> CstVisitor<F> for Formatter {
    type Error = Infallible;

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

    fn visit_at(&mut self, handle: AtHandle, view: AtView, tree: &F) -> Result<(), Self::Error> {
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

    fn visit_code_block_delimiter(
        &mut self,
        handle: CodeBlockDelimiterHandle,
        view: CodeBlockDelimiterView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_code_block_delimiter_super(handle, view, tree)
    }

    fn visit_code_block_line(
        &mut self,
        handle: CodeBlockLineHandle,
        view: CodeBlockLineView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_code_block_line_super(handle, view, tree)
    }

    fn visit_code_block_tail_common(
        &mut self,
        handle: CodeBlockTailCommonHandle,
        view: CodeBlockTailCommonView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_code_block_tail_common_super(handle, view, tree)
    }

    fn visit_code_block_tail_common_list(
        &mut self,
        handle: CodeBlockTailCommonListHandle,
        view: CodeBlockTailCommonListView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_code_block_tail_common_list_super(handle, view, tree)
    }

    fn visit_code_block_tail_common_opt(
        &mut self,
        handle: CodeBlockTailCommonOptHandle,
        view: WsHandle,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_code_block_tail_common_opt_super(handle, view, tree)
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

    fn visit_dot(&mut self, handle: DotHandle, view: DotView, tree: &F) -> Result<(), Self::Error> {
        self.visit_dot_super(handle, view, tree)
    }

    fn visit_end(&mut self, handle: EndHandle, view: EndView, tree: &F) -> Result<(), Self::Error> {
        self.visit_end_super(handle, view, tree)
    }

    fn visit_ext(&mut self, handle: ExtHandle, view: ExtView, tree: &F) -> Result<(), Self::Error> {
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

    fn visit_key(&mut self, handle: KeyHandle, view: KeyView, tree: &F) -> Result<(), Self::Error> {
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

    fn visit_named_code_block(
        &mut self,
        handle: NamedCodeBlockHandle,
        view: NamedCodeBlockView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_named_code_block_super(handle, view, tree)
    }

    fn visit_named_code_block_begin(
        &mut self,
        handle: NamedCodeBlockBeginHandle,
        view: NamedCodeBlockBeginView,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_named_code_block_begin_super(handle, view, tree)
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

    fn visit_str(&mut self, handle: StrHandle, view: StrView, tree: &F) -> Result<(), Self::Error> {
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

    fn visit_ws(&mut self, handle: WsHandle, view: WsView, tree: &F) -> Result<(), Self::Error> {
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
        data: swon_tree::prelude::TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_new_line_terminal_super(terminal, data, tree)
    }

    fn visit_whitespace_terminal(
        &mut self,
        terminal: Whitespace,
        data: swon_tree::prelude::TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_whitespace_terminal_super(terminal, data, tree)
    }

    fn visit_line_comment_terminal(
        &mut self,
        terminal: LineComment,
        data: swon_tree::prelude::TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_line_comment_terminal_super(terminal, data, tree)
    }

    fn visit_block_comment_terminal(
        &mut self,
        terminal: BlockComment,
        data: swon_tree::prelude::TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_block_comment_terminal_super(terminal, data, tree)
    }

    fn visit_integer_terminal(
        &mut self,
        terminal: Integer,
        data: swon_tree::prelude::TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_integer_terminal_super(terminal, data, tree)
    }

    fn visit_true_terminal(
        &mut self,
        terminal: True,
        data: swon_tree::prelude::TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_true_terminal_super(terminal, data, tree)
    }

    fn visit_false_terminal(
        &mut self,
        terminal: False,
        data: swon_tree::prelude::TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_false_terminal_super(terminal, data, tree)
    }

    fn visit_null_terminal(
        &mut self,
        terminal: Null,
        data: swon_tree::prelude::TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_null_terminal_super(terminal, data, tree)
    }

    fn visit_hole_terminal(
        &mut self,
        terminal: Hole,
        data: swon_tree::prelude::TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_hole_terminal_super(terminal, data, tree)
    }

    fn visit_quote_terminal(
        &mut self,
        terminal: Quote,
        data: swon_tree::prelude::TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_quote_terminal_super(terminal, data, tree)
    }

    fn visit_typed_quote_terminal(
        &mut self,
        terminal: TypedQuote,
        data: swon_tree::prelude::TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_typed_quote_terminal_super(terminal, data, tree)
    }

    fn visit_in_str_terminal(
        &mut self,
        terminal: InStr,
        data: swon_tree::prelude::TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_in_str_terminal_super(terminal, data, tree)
    }

    fn visit_text_terminal(
        &mut self,
        terminal: Text,
        data: swon_tree::prelude::TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_text_terminal_super(terminal, data, tree)
    }

    fn visit_named_code_terminal(
        &mut self,
        terminal: NamedCode,
        data: swon_tree::prelude::TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_named_code_terminal_super(terminal, data, tree)
    }

    fn visit_code_terminal(
        &mut self,
        terminal: Code,
        data: swon_tree::prelude::TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_code_terminal_super(terminal, data, tree)
    }

    fn visit_newline_terminal(
        &mut self,
        terminal: Newline,
        data: swon_tree::prelude::TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_newline_terminal_super(terminal, data, tree)
    }

    fn visit_ws_terminal(
        &mut self,
        terminal: Ws,
        data: swon_tree::prelude::TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_ws_terminal_super(terminal, data, tree)
    }

    fn visit_at_terminal(
        &mut self,
        terminal: At,
        data: swon_tree::prelude::TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_at_terminal_super(terminal, data, tree)
    }

    fn visit_dollar_terminal(
        &mut self,
        terminal: Dollar,
        data: swon_tree::prelude::TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_dollar_terminal_super(terminal, data, tree)
    }

    fn visit_dot_terminal(
        &mut self,
        terminal: Dot,
        data: swon_tree::prelude::TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_dot_terminal_super(terminal, data, tree)
    }

    fn visit_l_brace_terminal(
        &mut self,
        terminal: LBrace,
        data: swon_tree::prelude::TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_l_brace_terminal_super(terminal, data, tree)
    }

    fn visit_r_brace_terminal(
        &mut self,
        terminal: RBrace,
        data: swon_tree::prelude::TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_r_brace_terminal_super(terminal, data, tree)
    }

    fn visit_l_bracket_terminal(
        &mut self,
        terminal: LBracket,
        data: swon_tree::prelude::TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_l_bracket_terminal_super(terminal, data, tree)
    }

    fn visit_r_bracket_terminal(
        &mut self,
        terminal: RBracket,
        data: swon_tree::prelude::TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_r_bracket_terminal_super(terminal, data, tree)
    }

    fn visit_bind_terminal(
        &mut self,
        terminal: Bind,
        data: swon_tree::prelude::TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_bind_terminal_super(terminal, data, tree)
    }

    fn visit_comma_terminal(
        &mut self,
        terminal: Comma,
        data: swon_tree::prelude::TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_comma_terminal_super(terminal, data, tree)
    }

    fn visit_esc_terminal(
        &mut self,
        terminal: Esc,
        data: swon_tree::prelude::TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_esc_terminal_super(terminal, data, tree)
    }

    fn visit_text_start_terminal(
        &mut self,
        terminal: TextStart,
        data: swon_tree::prelude::TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_text_start_terminal_super(terminal, data, tree)
    }

    fn visit_ident_terminal(
        &mut self,
        terminal: Ident,
        data: swon_tree::prelude::TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_ident_terminal_super(terminal, data, tree)
    }

    fn visit_named_code_block_begin_terminal(
        &mut self,
        terminal: NamedCodeBlockBegin,
        data: swon_tree::prelude::TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_named_code_block_begin_terminal_super(terminal, data, tree)
    }

    fn visit_code_block_delimiter_terminal(
        &mut self,
        terminal: CodeBlockDelimiter,
        data: swon_tree::prelude::TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_code_block_delimiter_terminal_super(terminal, data, tree)
    }

    fn visit_code_block_line_terminal(
        &mut self,
        terminal: CodeBlockLine,
        data: swon_tree::prelude::TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_code_block_line_terminal_super(terminal, data, tree)
    }

    fn visit_non_terminal(
        &mut self,
        id: CstNodeId,
        kind: swon_tree::node_kind::NonTerminalKind,
        data: swon_tree::prelude::NonTerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_non_terminal_super(id, kind, data, tree)
    }

    fn visit_non_terminal_close(
        &mut self,
        id: CstNodeId,
        kind: swon_tree::node_kind::NonTerminalKind,
        data: swon_tree::prelude::NonTerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_non_terminal_close_super(id, kind, data, tree)
    }

    fn visit_terminal(
        &mut self,
        id: CstNodeId,
        kind: swon_tree::node_kind::TerminalKind,
        data: swon_tree::prelude::TerminalData,
        tree: &F,
    ) -> Result<(), Self::Error> {
        self.visit_terminal_super(id, kind, data, tree)
    }

    fn then_construct_error(
        &mut self,
        node_data: Option<swon_tree::CstNode>,
        parent: CstNodeId,
        kind: swon_tree::NodeKind,
        error: swon_tree::CstConstructError,
        tree: &F,
    ) -> Result<(), Self::Error> {
        let _error = error;
        self.recover_error(node_data, parent, kind, tree)
    }
}
