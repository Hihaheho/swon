use std::convert::Infallible;

use swon_tree::{
    Cst,
    action::CstCommands,
    tree::CstNodeId,
    visitor::{CstHandleSuper, CstVisitor},
};

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

impl CstVisitor for Formatter {
    type Error = Infallible;

    fn then_construct_error(
        &mut self,
        node_data: Option<swon_tree::CstNode>,
        parent: swon_tree::tree::CstNodeId,
        kind: swon_tree::NodeKind,
        _error: swon_tree::CstConstructError,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.recover_error(node_data, parent, kind, tree)
    }

    fn visit_array(
        &mut self,
        handle: swon_tree::ast::ArrayHandle,
        view: swon_tree::ast::ArrayView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_array_super(view, tree)
    }

    fn visit_array_begin(
        &mut self,
        handle: swon_tree::ast::ArrayBeginHandle,
        view: swon_tree::ast::ArrayBeginView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_array_begin_super(view, tree)
    }

    fn visit_array_end(
        &mut self,
        handle: swon_tree::ast::ArrayEndHandle,
        view: swon_tree::ast::ArrayEndView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_array_end_super(view, tree)
    }

    fn visit_array_list(
        &mut self,
        handle: swon_tree::ast::ArrayListHandle,
        view: swon_tree::ast::ArrayListView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_array_list_super(view, tree)
    }

    fn visit_array_marker(
        &mut self,
        handle: swon_tree::ast::ArrayMarkerHandle,
        view: swon_tree::ast::ArrayMarkerView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_array_marker_super(view, tree)
    }

    fn visit_array_marker_opt(
        &mut self,
        handle: swon_tree::ast::ArrayMarkerOptHandle,
        view: swon_tree::ast::IntegerHandle,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_array_marker_opt_super(view, tree)
    }

    fn visit_array_opt(
        &mut self,
        handle: swon_tree::ast::ArrayOptHandle,
        view: swon_tree::ast::CommaHandle,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_array_opt_super(view, tree)
    }

    fn visit_at(
        &mut self,
        handle: swon_tree::ast::AtHandle,
        view: swon_tree::ast::AtView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_at_super(view, tree)
    }

    fn visit_begin(
        &mut self,
        handle: swon_tree::ast::BeginHandle,
        view: swon_tree::ast::BeginView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_begin_super(view, tree)
    }

    fn visit_bind(
        &mut self,
        handle: swon_tree::ast::BindHandle,
        view: swon_tree::ast::BindView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_bind_super(view, tree)
    }

    fn visit_binding(
        &mut self,
        handle: swon_tree::ast::BindingHandle,
        view: swon_tree::ast::BindingView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_binding_super(view, tree)
    }

    fn visit_binding_rhs(
        &mut self,
        handle: swon_tree::ast::BindingRhsHandle,
        view: swon_tree::ast::BindingRhsView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_binding_rhs_super(view, tree)
    }

    fn visit_boolean(
        &mut self,
        handle: swon_tree::ast::BooleanHandle,
        view: swon_tree::ast::BooleanView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_boolean_super(view, tree)
    }

    fn visit_code(
        &mut self,
        handle: swon_tree::ast::CodeHandle,
        view: swon_tree::ast::CodeView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_code_super(view, tree)
    }

    fn visit_code_block(
        &mut self,
        handle: swon_tree::ast::CodeBlockHandle,
        view: swon_tree::ast::CodeBlockView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_code_block_super(view, tree)
    }

    fn visit_code_block_delimiter(
        &mut self,
        handle: swon_tree::ast::CodeBlockDelimiterHandle,
        view: swon_tree::ast::CodeBlockDelimiterView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_code_block_delimiter_super(view, tree)
    }

    fn visit_code_block_line(
        &mut self,
        handle: swon_tree::ast::CodeBlockLineHandle,
        view: swon_tree::ast::CodeBlockLineView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_code_block_line_super(view, tree)
    }

    fn visit_code_block_tail_common(
        &mut self,
        handle: swon_tree::ast::CodeBlockTailCommonHandle,
        view: swon_tree::ast::CodeBlockTailCommonView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_code_block_tail_common_super(view, tree)
    }

    fn visit_code_block_tail_common_list(
        &mut self,
        handle: swon_tree::ast::CodeBlockTailCommonListHandle,
        view: swon_tree::ast::CodeBlockTailCommonListView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_code_block_tail_common_list_super(view, tree)
    }

    fn visit_code_block_tail_common_opt(
        &mut self,
        handle: swon_tree::ast::CodeBlockTailCommonOptHandle,
        view: swon_tree::ast::WsHandle,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_code_block_tail_common_opt_super(view, tree)
    }

    fn visit_comma(
        &mut self,
        handle: swon_tree::ast::CommaHandle,
        view: swon_tree::ast::CommaView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_comma_super(view, tree)
    }

    fn visit_continue(
        &mut self,
        handle: swon_tree::ast::ContinueHandle,
        view: swon_tree::ast::ContinueView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_continue_super(view, tree)
    }

    fn visit_dot(
        &mut self,
        handle: swon_tree::ast::DotHandle,
        view: swon_tree::ast::DotView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_dot_super(view, tree)
    }

    fn visit_end(
        &mut self,
        handle: swon_tree::ast::EndHandle,
        view: swon_tree::ast::EndView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_end_super(view, tree)
    }

    fn visit_ext(
        &mut self,
        handle: swon_tree::ast::ExtHandle,
        view: swon_tree::ast::ExtView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_ext_super(view, tree)
    }

    fn visit_extension_name_space(
        &mut self,
        handle: swon_tree::ast::ExtensionNameSpaceHandle,
        view: swon_tree::ast::ExtensionNameSpaceView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_extension_name_space_super(view, tree)
    }

    fn visit_false(
        &mut self,
        handle: swon_tree::ast::FalseHandle,
        view: swon_tree::ast::FalseView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_false_super(view, tree)
    }

    fn visit_hole(
        &mut self,
        handle: swon_tree::ast::HoleHandle,
        view: swon_tree::ast::HoleView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_hole_super(view, tree)
    }

    fn visit_ident(
        &mut self,
        handle: swon_tree::ast::IdentHandle,
        view: swon_tree::ast::IdentView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_ident_super(view, tree)
    }

    fn visit_in_str(
        &mut self,
        handle: swon_tree::ast::InStrHandle,
        view: swon_tree::ast::InStrView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_in_str_super(view, tree)
    }

    fn visit_integer(
        &mut self,
        handle: swon_tree::ast::IntegerHandle,
        view: swon_tree::ast::IntegerView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_integer_super(view, tree)
    }

    fn visit_key(
        &mut self,
        handle: swon_tree::ast::KeyHandle,
        view: swon_tree::ast::KeyView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_key_super(view, tree)
    }

    fn visit_key_base(
        &mut self,
        handle: swon_tree::ast::KeyBaseHandle,
        view: swon_tree::ast::KeyBaseView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_key_base_super(view, tree)
    }

    fn visit_key_opt(
        &mut self,
        handle: swon_tree::ast::KeyOptHandle,
        view: swon_tree::ast::ArrayMarkerHandle,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_key_opt_super(view, tree)
    }

    fn visit_keys(
        &mut self,
        handle: swon_tree::ast::KeysHandle,
        view: swon_tree::ast::KeysView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_keys_super(view, tree)
    }

    fn visit_keys_list(
        &mut self,
        handle: swon_tree::ast::KeysListHandle,
        view: swon_tree::ast::KeysListView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_keys_list_super(view, tree)
    }

    fn visit_named_code(
        &mut self,
        handle: swon_tree::ast::NamedCodeHandle,
        view: swon_tree::ast::NamedCodeView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_named_code_super(view, tree)
    }

    fn visit_named_code_block(
        &mut self,
        handle: swon_tree::ast::NamedCodeBlockHandle,
        view: swon_tree::ast::NamedCodeBlockView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_named_code_block_super(view, tree)
    }

    fn visit_named_code_block_begin(
        &mut self,
        handle: swon_tree::ast::NamedCodeBlockBeginHandle,
        view: swon_tree::ast::NamedCodeBlockBeginView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_named_code_block_begin_super(view, tree)
    }

    fn visit_newline(
        &mut self,
        handle: swon_tree::ast::NewlineHandle,
        view: swon_tree::ast::NewlineView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_newline_super(view, tree)
    }

    fn visit_null(
        &mut self,
        handle: swon_tree::ast::NullHandle,
        view: swon_tree::ast::NullView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_null_super(view, tree)
    }

    fn visit_object(
        &mut self,
        handle: swon_tree::ast::ObjectHandle,
        view: swon_tree::ast::ObjectView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_object_super(view, tree)
    }

    fn visit_object_list(
        &mut self,
        handle: swon_tree::ast::ObjectListHandle,
        view: swon_tree::ast::ObjectListView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_object_list_super(view, tree)
    }

    fn visit_object_opt(
        &mut self,
        handle: swon_tree::ast::ObjectOptHandle,
        view: swon_tree::ast::CommaHandle,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_object_opt_super(view, tree)
    }

    fn visit_quote(
        &mut self,
        handle: swon_tree::ast::QuoteHandle,
        view: swon_tree::ast::QuoteView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_quote_super(view, tree)
    }

    fn visit_section(
        &mut self,
        handle: swon_tree::ast::SectionHandle,
        view: swon_tree::ast::SectionView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_section_super(view, tree)
    }

    fn visit_section_binding(
        &mut self,
        handle: swon_tree::ast::SectionBindingHandle,
        view: swon_tree::ast::SectionBindingView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_section_binding_super(view, tree)
    }

    fn visit_section_body(
        &mut self,
        handle: swon_tree::ast::SectionBodyHandle,
        view: swon_tree::ast::SectionBodyView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_section_body_super(view, tree)
    }

    fn visit_section_body_list(
        &mut self,
        handle: swon_tree::ast::SectionBodyListHandle,
        view: swon_tree::ast::SectionBodyListView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_section_body_list_super(view, tree)
    }

    fn visit_str(
        &mut self,
        handle: swon_tree::ast::StrHandle,
        view: swon_tree::ast::StrView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_str_super(view, tree)
    }

    fn visit_str_continues(
        &mut self,
        handle: swon_tree::ast::StrContinuesHandle,
        view: swon_tree::ast::StrContinuesView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_str_continues_super(view, tree)
    }

    fn visit_str_continues_list(
        &mut self,
        handle: swon_tree::ast::StrContinuesListHandle,
        view: swon_tree::ast::StrContinuesListView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_str_continues_list_super(view, tree)
    }

    fn visit_swon(
        &mut self,
        handle: swon_tree::ast::SwonHandle,
        view: swon_tree::ast::SwonView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_swon_super(view, tree)
    }

    fn visit_swon_bindings(
        &mut self,
        handle: swon_tree::ast::SwonBindingsHandle,
        view: swon_tree::ast::SwonBindingsView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_swon_bindings_super(view, tree)
    }

    fn visit_swon_sections(
        &mut self,
        handle: swon_tree::ast::SwonSectionsHandle,
        view: swon_tree::ast::SwonSectionsView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_swon_sections_super(view, tree)
    }

    fn visit_text(
        &mut self,
        handle: swon_tree::ast::TextHandle,
        view: swon_tree::ast::TextView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_text_super(view, tree)
    }

    fn visit_text_binding(
        &mut self,
        handle: swon_tree::ast::TextBindingHandle,
        view: swon_tree::ast::TextBindingView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_text_binding_super(view, tree)
    }

    fn visit_text_binding_opt(
        &mut self,
        handle: swon_tree::ast::TextBindingOptHandle,
        view: swon_tree::ast::WsHandle,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_text_binding_opt_super(view, tree)
    }

    fn visit_text_start(
        &mut self,
        handle: swon_tree::ast::TextStartHandle,
        view: swon_tree::ast::TextStartView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_text_start_super(view, tree)
    }

    fn visit_true(
        &mut self,
        handle: swon_tree::ast::TrueHandle,
        view: swon_tree::ast::TrueView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_true_super(view, tree)
    }

    fn visit_typed_quote(
        &mut self,
        handle: swon_tree::ast::TypedQuoteHandle,
        view: swon_tree::ast::TypedQuoteView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_typed_quote_super(view, tree)
    }

    fn visit_typed_str(
        &mut self,
        handle: swon_tree::ast::TypedStrHandle,
        view: swon_tree::ast::TypedStrView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_typed_str_super(view, tree)
    }

    fn visit_value(
        &mut self,
        handle: swon_tree::ast::ValueHandle,
        view: swon_tree::ast::ValueView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_value_super(view, tree)
    }

    fn visit_value_binding(
        &mut self,
        handle: swon_tree::ast::ValueBindingHandle,
        view: swon_tree::ast::ValueBindingView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_value_binding_super(view, tree)
    }

    fn visit_ws(
        &mut self,
        handle: swon_tree::ast::WsHandle,
        view: swon_tree::ast::WsView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_ws_super(view, tree)
    }

    fn visit_root(
        &mut self,
        handle: swon_tree::ast::RootHandle,
        view: swon_tree::ast::RootView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_root_super(view, tree)
    }

    fn visit_new_line_terminal(
        &mut self,
        terminal: swon_tree::ast::NewLine,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_new_line_terminal_super(terminal, data, tree)
    }

    fn visit_whitespace_terminal(
        &mut self,
        terminal: swon_tree::ast::Whitespace,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_whitespace_terminal_super(terminal, data, tree)
    }

    fn visit_line_comment_terminal(
        &mut self,
        terminal: swon_tree::ast::LineComment,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_line_comment_terminal_super(terminal, data, tree)
    }

    fn visit_block_comment_terminal(
        &mut self,
        terminal: swon_tree::ast::BlockComment,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_block_comment_terminal_super(terminal, data, tree)
    }

    fn visit_integer_terminal(
        &mut self,
        terminal: swon_tree::ast::Integer,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_integer_terminal_super(terminal, data, tree)
    }

    fn visit_true_terminal(
        &mut self,
        terminal: swon_tree::ast::True,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_true_terminal_super(terminal, data, tree)
    }

    fn visit_false_terminal(
        &mut self,
        terminal: swon_tree::ast::False,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_false_terminal_super(terminal, data, tree)
    }

    fn visit_null_terminal(
        &mut self,
        terminal: swon_tree::ast::Null,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_null_terminal_super(terminal, data, tree)
    }

    fn visit_hole_terminal(
        &mut self,
        terminal: swon_tree::ast::Hole,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_hole_terminal_super(terminal, data, tree)
    }

    fn visit_quote_terminal(
        &mut self,
        terminal: swon_tree::ast::Quote,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_quote_terminal_super(terminal, data, tree)
    }

    fn visit_typed_quote_terminal(
        &mut self,
        terminal: swon_tree::ast::TypedQuote,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_typed_quote_terminal_super(terminal, data, tree)
    }

    fn visit_in_str_terminal(
        &mut self,
        terminal: swon_tree::ast::InStr,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_in_str_terminal_super(terminal, data, tree)
    }

    fn visit_text_terminal(
        &mut self,
        terminal: swon_tree::ast::Text,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_text_terminal_super(terminal, data, tree)
    }

    fn visit_named_code_terminal(
        &mut self,
        terminal: swon_tree::ast::NamedCode,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_named_code_terminal_super(terminal, data, tree)
    }

    fn visit_code_terminal(
        &mut self,
        terminal: swon_tree::ast::Code,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_code_terminal_super(terminal, data, tree)
    }

    fn visit_newline_terminal(
        &mut self,
        terminal: swon_tree::ast::Newline,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_newline_terminal_super(terminal, data, tree)
    }

    fn visit_ws_terminal(
        &mut self,
        terminal: swon_tree::ast::Ws,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_ws_terminal_super(terminal, data, tree)
    }

    fn visit_at_terminal(
        &mut self,
        terminal: swon_tree::ast::At,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_at_terminal_super(terminal, data, tree)
    }

    fn visit_dollar_terminal(
        &mut self,
        terminal: swon_tree::ast::Dollar,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_dollar_terminal_super(terminal, data, tree)
    }

    fn visit_dot_terminal(
        &mut self,
        terminal: swon_tree::ast::Dot,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_dot_terminal_super(terminal, data, tree)
    }

    fn visit_l_brace_terminal(
        &mut self,
        terminal: swon_tree::ast::LBrace,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_l_brace_terminal_super(terminal, data, tree)
    }

    fn visit_r_brace_terminal(
        &mut self,
        terminal: swon_tree::ast::RBrace,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_r_brace_terminal_super(terminal, data, tree)
    }

    fn visit_l_bracket_terminal(
        &mut self,
        terminal: swon_tree::ast::LBracket,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_l_bracket_terminal_super(terminal, data, tree)
    }

    fn visit_r_bracket_terminal(
        &mut self,
        terminal: swon_tree::ast::RBracket,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_r_bracket_terminal_super(terminal, data, tree)
    }

    fn visit_bind_terminal(
        &mut self,
        terminal: swon_tree::ast::Bind,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_bind_terminal_super(terminal, data, tree)
    }

    fn visit_comma_terminal(
        &mut self,
        terminal: swon_tree::ast::Comma,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_comma_terminal_super(terminal, data, tree)
    }

    fn visit_esc_terminal(
        &mut self,
        terminal: swon_tree::ast::Esc,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_esc_terminal_super(terminal, data, tree)
    }

    fn visit_text_start_terminal(
        &mut self,
        terminal: swon_tree::ast::TextStart,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_text_start_terminal_super(terminal, data, tree)
    }

    fn visit_ident_terminal(
        &mut self,
        terminal: swon_tree::ast::Ident,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_ident_terminal_super(terminal, data, tree)
    }

    fn visit_named_code_block_begin_terminal(
        &mut self,
        terminal: swon_tree::ast::NamedCodeBlockBegin,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_named_code_block_begin_terminal_super(terminal, data, tree)
    }

    fn visit_code_block_delimiter_terminal(
        &mut self,
        terminal: swon_tree::ast::CodeBlockDelimiter,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_code_block_delimiter_terminal_super(terminal, data, tree)
    }

    fn visit_code_block_line_terminal(
        &mut self,
        terminal: swon_tree::ast::CodeBlockLine,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_code_block_line_terminal_super(terminal, data, tree)
    }

    fn visit_non_terminal(
        &mut self,
        id: swon_tree::tree::CstNodeId,
        kind: swon_tree::nodes::NonTerminalKind,
        data: swon_tree::tree::NonTerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_non_terminal_super(id, kind, data, tree)
    }

    fn visit_non_terminal_close(
        &mut self,
        id: swon_tree::tree::CstNodeId,
        kind: swon_tree::nodes::NonTerminalKind,
        data: swon_tree::tree::NonTerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_non_terminal_close_super(id, kind, data, tree)
    }

    fn visit_terminal(
        &mut self,
        id: swon_tree::tree::CstNodeId,
        kind: swon_tree::nodes::TerminalKind,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_terminal_super(id, kind, data, tree)
    }
}
