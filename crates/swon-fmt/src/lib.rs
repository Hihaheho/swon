use std::convert::Infallible;

use swon_parol::{
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
        node_data: Option<swon_parol::CstNode>,
        parent: swon_parol::tree::CstNodeId,
        kind: swon_parol::NodeKind,
        _error: swon_parol::CstConstructError,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.recover_error(node_data, parent, kind, tree)
    }

    fn visit_array(
        &mut self,
        handle: swon_parol::ast::ArrayHandle,
        view: swon_parol::ast::ArrayView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_array_super(view, tree)
    }

    fn visit_array_begin(
        &mut self,
        handle: swon_parol::ast::ArrayBeginHandle,
        view: swon_parol::ast::ArrayBeginView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_array_begin_super(view, tree)
    }

    fn visit_array_end(
        &mut self,
        handle: swon_parol::ast::ArrayEndHandle,
        view: swon_parol::ast::ArrayEndView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_array_end_super(view, tree)
    }

    fn visit_array_list(
        &mut self,
        handle: swon_parol::ast::ArrayListHandle,
        view: swon_parol::ast::ArrayListView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_array_list_super(view, tree)
    }

    fn visit_array_marker(
        &mut self,
        handle: swon_parol::ast::ArrayMarkerHandle,
        view: swon_parol::ast::ArrayMarkerView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_array_marker_super(view, tree)
    }

    fn visit_array_marker_opt(
        &mut self,
        handle: swon_parol::ast::ArrayMarkerOptHandle,
        view: swon_parol::ast::IntegerHandle,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_array_marker_opt_super(view, tree)
    }

    fn visit_array_opt(
        &mut self,
        handle: swon_parol::ast::ArrayOptHandle,
        view: swon_parol::ast::CommaHandle,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_array_opt_super(view, tree)
    }

    fn visit_at(
        &mut self,
        handle: swon_parol::ast::AtHandle,
        view: swon_parol::ast::AtView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_at_super(view, tree)
    }

    fn visit_begin(
        &mut self,
        handle: swon_parol::ast::BeginHandle,
        view: swon_parol::ast::BeginView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_begin_super(view, tree)
    }

    fn visit_bind(
        &mut self,
        handle: swon_parol::ast::BindHandle,
        view: swon_parol::ast::BindView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_bind_super(view, tree)
    }

    fn visit_binding(
        &mut self,
        handle: swon_parol::ast::BindingHandle,
        view: swon_parol::ast::BindingView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_binding_super(view, tree)
    }

    fn visit_binding_rhs(
        &mut self,
        handle: swon_parol::ast::BindingRhsHandle,
        view: swon_parol::ast::BindingRhsView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_binding_rhs_super(view, tree)
    }

    fn visit_boolean(
        &mut self,
        handle: swon_parol::ast::BooleanHandle,
        view: swon_parol::ast::BooleanView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_boolean_super(view, tree)
    }

    fn visit_code(
        &mut self,
        handle: swon_parol::ast::CodeHandle,
        view: swon_parol::ast::CodeView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_code_super(view, tree)
    }

    fn visit_code_block(
        &mut self,
        handle: swon_parol::ast::CodeBlockHandle,
        view: swon_parol::ast::CodeBlockView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_code_block_super(view, tree)
    }

    fn visit_code_block_delimiter(
        &mut self,
        handle: swon_parol::ast::CodeBlockDelimiterHandle,
        view: swon_parol::ast::CodeBlockDelimiterView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_code_block_delimiter_super(view, tree)
    }

    fn visit_code_block_line(
        &mut self,
        handle: swon_parol::ast::CodeBlockLineHandle,
        view: swon_parol::ast::CodeBlockLineView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_code_block_line_super(view, tree)
    }

    fn visit_code_block_tail_common(
        &mut self,
        handle: swon_parol::ast::CodeBlockTailCommonHandle,
        view: swon_parol::ast::CodeBlockTailCommonView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_code_block_tail_common_super(view, tree)
    }

    fn visit_code_block_tail_common_list(
        &mut self,
        handle: swon_parol::ast::CodeBlockTailCommonListHandle,
        view: swon_parol::ast::CodeBlockTailCommonListView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_code_block_tail_common_list_super(view, tree)
    }

    fn visit_code_block_tail_common_opt(
        &mut self,
        handle: swon_parol::ast::CodeBlockTailCommonOptHandle,
        view: swon_parol::ast::WsHandle,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_code_block_tail_common_opt_super(view, tree)
    }

    fn visit_comma(
        &mut self,
        handle: swon_parol::ast::CommaHandle,
        view: swon_parol::ast::CommaView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_comma_super(view, tree)
    }

    fn visit_continue(
        &mut self,
        handle: swon_parol::ast::ContinueHandle,
        view: swon_parol::ast::ContinueView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_continue_super(view, tree)
    }

    fn visit_dot(
        &mut self,
        handle: swon_parol::ast::DotHandle,
        view: swon_parol::ast::DotView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_dot_super(view, tree)
    }

    fn visit_end(
        &mut self,
        handle: swon_parol::ast::EndHandle,
        view: swon_parol::ast::EndView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_end_super(view, tree)
    }

    fn visit_ext(
        &mut self,
        handle: swon_parol::ast::ExtHandle,
        view: swon_parol::ast::ExtView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_ext_super(view, tree)
    }

    fn visit_extension_name_space(
        &mut self,
        handle: swon_parol::ast::ExtensionNameSpaceHandle,
        view: swon_parol::ast::ExtensionNameSpaceView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_extension_name_space_super(view, tree)
    }

    fn visit_false(
        &mut self,
        handle: swon_parol::ast::FalseHandle,
        view: swon_parol::ast::FalseView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_false_super(view, tree)
    }

    fn visit_hole(
        &mut self,
        handle: swon_parol::ast::HoleHandle,
        view: swon_parol::ast::HoleView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_hole_super(view, tree)
    }

    fn visit_ident(
        &mut self,
        handle: swon_parol::ast::IdentHandle,
        view: swon_parol::ast::IdentView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_ident_super(view, tree)
    }

    fn visit_in_str(
        &mut self,
        handle: swon_parol::ast::InStrHandle,
        view: swon_parol::ast::InStrView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_in_str_super(view, tree)
    }

    fn visit_integer(
        &mut self,
        handle: swon_parol::ast::IntegerHandle,
        view: swon_parol::ast::IntegerView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_integer_super(view, tree)
    }

    fn visit_key(
        &mut self,
        handle: swon_parol::ast::KeyHandle,
        view: swon_parol::ast::KeyView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_key_super(view, tree)
    }

    fn visit_key_base(
        &mut self,
        handle: swon_parol::ast::KeyBaseHandle,
        view: swon_parol::ast::KeyBaseView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_key_base_super(view, tree)
    }

    fn visit_key_opt(
        &mut self,
        handle: swon_parol::ast::KeyOptHandle,
        view: swon_parol::ast::ArrayMarkerHandle,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_key_opt_super(view, tree)
    }

    fn visit_keys(
        &mut self,
        handle: swon_parol::ast::KeysHandle,
        view: swon_parol::ast::KeysView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_keys_super(view, tree)
    }

    fn visit_keys_list(
        &mut self,
        handle: swon_parol::ast::KeysListHandle,
        view: swon_parol::ast::KeysListView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_keys_list_super(view, tree)
    }

    fn visit_named_code(
        &mut self,
        handle: swon_parol::ast::NamedCodeHandle,
        view: swon_parol::ast::NamedCodeView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_named_code_super(view, tree)
    }

    fn visit_named_code_block(
        &mut self,
        handle: swon_parol::ast::NamedCodeBlockHandle,
        view: swon_parol::ast::NamedCodeBlockView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_named_code_block_super(view, tree)
    }

    fn visit_named_code_block_begin(
        &mut self,
        handle: swon_parol::ast::NamedCodeBlockBeginHandle,
        view: swon_parol::ast::NamedCodeBlockBeginView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_named_code_block_begin_super(view, tree)
    }

    fn visit_newline(
        &mut self,
        handle: swon_parol::ast::NewlineHandle,
        view: swon_parol::ast::NewlineView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_newline_super(view, tree)
    }

    fn visit_null(
        &mut self,
        handle: swon_parol::ast::NullHandle,
        view: swon_parol::ast::NullView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_null_super(view, tree)
    }

    fn visit_object(
        &mut self,
        handle: swon_parol::ast::ObjectHandle,
        view: swon_parol::ast::ObjectView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_object_super(view, tree)
    }

    fn visit_object_list(
        &mut self,
        handle: swon_parol::ast::ObjectListHandle,
        view: swon_parol::ast::ObjectListView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_object_list_super(view, tree)
    }

    fn visit_object_opt(
        &mut self,
        handle: swon_parol::ast::ObjectOptHandle,
        view: swon_parol::ast::CommaHandle,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_object_opt_super(view, tree)
    }

    fn visit_quote(
        &mut self,
        handle: swon_parol::ast::QuoteHandle,
        view: swon_parol::ast::QuoteView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_quote_super(view, tree)
    }

    fn visit_section(
        &mut self,
        handle: swon_parol::ast::SectionHandle,
        view: swon_parol::ast::SectionView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_section_super(view, tree)
    }

    fn visit_section_binding(
        &mut self,
        handle: swon_parol::ast::SectionBindingHandle,
        view: swon_parol::ast::SectionBindingView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_section_binding_super(view, tree)
    }

    fn visit_section_body(
        &mut self,
        handle: swon_parol::ast::SectionBodyHandle,
        view: swon_parol::ast::SectionBodyView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_section_body_super(view, tree)
    }

    fn visit_section_body_list(
        &mut self,
        handle: swon_parol::ast::SectionBodyListHandle,
        view: swon_parol::ast::SectionBodyListView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_section_body_list_super(view, tree)
    }

    fn visit_str(
        &mut self,
        handle: swon_parol::ast::StrHandle,
        view: swon_parol::ast::StrView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_str_super(view, tree)
    }

    fn visit_str_continues(
        &mut self,
        handle: swon_parol::ast::StrContinuesHandle,
        view: swon_parol::ast::StrContinuesView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_str_continues_super(view, tree)
    }

    fn visit_str_continues_list(
        &mut self,
        handle: swon_parol::ast::StrContinuesListHandle,
        view: swon_parol::ast::StrContinuesListView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_str_continues_list_super(view, tree)
    }

    fn visit_swon(
        &mut self,
        handle: swon_parol::ast::SwonHandle,
        view: swon_parol::ast::SwonView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_swon_super(view, tree)
    }

    fn visit_swon_bindings(
        &mut self,
        handle: swon_parol::ast::SwonBindingsHandle,
        view: swon_parol::ast::SwonBindingsView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_swon_bindings_super(view, tree)
    }

    fn visit_swon_sections(
        &mut self,
        handle: swon_parol::ast::SwonSectionsHandle,
        view: swon_parol::ast::SwonSectionsView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_swon_sections_super(view, tree)
    }

    fn visit_text(
        &mut self,
        handle: swon_parol::ast::TextHandle,
        view: swon_parol::ast::TextView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_text_super(view, tree)
    }

    fn visit_text_binding(
        &mut self,
        handle: swon_parol::ast::TextBindingHandle,
        view: swon_parol::ast::TextBindingView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_text_binding_super(view, tree)
    }

    fn visit_text_binding_opt(
        &mut self,
        handle: swon_parol::ast::TextBindingOptHandle,
        view: swon_parol::ast::WsHandle,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_text_binding_opt_super(view, tree)
    }

    fn visit_text_start(
        &mut self,
        handle: swon_parol::ast::TextStartHandle,
        view: swon_parol::ast::TextStartView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_text_start_super(view, tree)
    }

    fn visit_true(
        &mut self,
        handle: swon_parol::ast::TrueHandle,
        view: swon_parol::ast::TrueView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_true_super(view, tree)
    }

    fn visit_typed_quote(
        &mut self,
        handle: swon_parol::ast::TypedQuoteHandle,
        view: swon_parol::ast::TypedQuoteView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_typed_quote_super(view, tree)
    }

    fn visit_typed_str(
        &mut self,
        handle: swon_parol::ast::TypedStrHandle,
        view: swon_parol::ast::TypedStrView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_typed_str_super(view, tree)
    }

    fn visit_value(
        &mut self,
        handle: swon_parol::ast::ValueHandle,
        view: swon_parol::ast::ValueView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_value_super(view, tree)
    }

    fn visit_value_binding(
        &mut self,
        handle: swon_parol::ast::ValueBindingHandle,
        view: swon_parol::ast::ValueBindingView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_value_binding_super(view, tree)
    }

    fn visit_ws(
        &mut self,
        handle: swon_parol::ast::WsHandle,
        view: swon_parol::ast::WsView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_ws_super(view, tree)
    }

    fn visit_root(
        &mut self,
        handle: swon_parol::ast::RootHandle,
        view: swon_parol::ast::RootView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_root_super(view, tree)
    }

    fn visit_new_line_terminal(
        &mut self,
        terminal: swon_parol::ast::NewLine,
        data: swon_parol::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_new_line_terminal_super(terminal, data, tree)
    }

    fn visit_whitespace_terminal(
        &mut self,
        terminal: swon_parol::ast::Whitespace,
        data: swon_parol::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_whitespace_terminal_super(terminal, data, tree)
    }

    fn visit_line_comment_terminal(
        &mut self,
        terminal: swon_parol::ast::LineComment,
        data: swon_parol::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_line_comment_terminal_super(terminal, data, tree)
    }

    fn visit_block_comment_terminal(
        &mut self,
        terminal: swon_parol::ast::BlockComment,
        data: swon_parol::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_block_comment_terminal_super(terminal, data, tree)
    }

    fn visit_integer_terminal(
        &mut self,
        terminal: swon_parol::ast::Integer,
        data: swon_parol::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_integer_terminal_super(terminal, data, tree)
    }

    fn visit_true_terminal(
        &mut self,
        terminal: swon_parol::ast::True,
        data: swon_parol::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_true_terminal_super(terminal, data, tree)
    }

    fn visit_false_terminal(
        &mut self,
        terminal: swon_parol::ast::False,
        data: swon_parol::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_false_terminal_super(terminal, data, tree)
    }

    fn visit_null_terminal(
        &mut self,
        terminal: swon_parol::ast::Null,
        data: swon_parol::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_null_terminal_super(terminal, data, tree)
    }

    fn visit_hole_terminal(
        &mut self,
        terminal: swon_parol::ast::Hole,
        data: swon_parol::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_hole_terminal_super(terminal, data, tree)
    }

    fn visit_quote_terminal(
        &mut self,
        terminal: swon_parol::ast::Quote,
        data: swon_parol::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_quote_terminal_super(terminal, data, tree)
    }

    fn visit_typed_quote_terminal(
        &mut self,
        terminal: swon_parol::ast::TypedQuote,
        data: swon_parol::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_typed_quote_terminal_super(terminal, data, tree)
    }

    fn visit_in_str_terminal(
        &mut self,
        terminal: swon_parol::ast::InStr,
        data: swon_parol::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_in_str_terminal_super(terminal, data, tree)
    }

    fn visit_text_terminal(
        &mut self,
        terminal: swon_parol::ast::Text,
        data: swon_parol::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_text_terminal_super(terminal, data, tree)
    }

    fn visit_named_code_terminal(
        &mut self,
        terminal: swon_parol::ast::NamedCode,
        data: swon_parol::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_named_code_terminal_super(terminal, data, tree)
    }

    fn visit_code_terminal(
        &mut self,
        terminal: swon_parol::ast::Code,
        data: swon_parol::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_code_terminal_super(terminal, data, tree)
    }

    fn visit_newline_terminal(
        &mut self,
        terminal: swon_parol::ast::Newline,
        data: swon_parol::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_newline_terminal_super(terminal, data, tree)
    }

    fn visit_ws_terminal(
        &mut self,
        terminal: swon_parol::ast::Ws,
        data: swon_parol::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_ws_terminal_super(terminal, data, tree)
    }

    fn visit_at_terminal(
        &mut self,
        terminal: swon_parol::ast::At,
        data: swon_parol::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_at_terminal_super(terminal, data, tree)
    }

    fn visit_dollar_terminal(
        &mut self,
        terminal: swon_parol::ast::Dollar,
        data: swon_parol::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_dollar_terminal_super(terminal, data, tree)
    }

    fn visit_dot_terminal(
        &mut self,
        terminal: swon_parol::ast::Dot,
        data: swon_parol::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_dot_terminal_super(terminal, data, tree)
    }

    fn visit_l_brace_terminal(
        &mut self,
        terminal: swon_parol::ast::LBrace,
        data: swon_parol::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_l_brace_terminal_super(terminal, data, tree)
    }

    fn visit_r_brace_terminal(
        &mut self,
        terminal: swon_parol::ast::RBrace,
        data: swon_parol::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_r_brace_terminal_super(terminal, data, tree)
    }

    fn visit_l_bracket_terminal(
        &mut self,
        terminal: swon_parol::ast::LBracket,
        data: swon_parol::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_l_bracket_terminal_super(terminal, data, tree)
    }

    fn visit_r_bracket_terminal(
        &mut self,
        terminal: swon_parol::ast::RBracket,
        data: swon_parol::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_r_bracket_terminal_super(terminal, data, tree)
    }

    fn visit_bind_terminal(
        &mut self,
        terminal: swon_parol::ast::Bind,
        data: swon_parol::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_bind_terminal_super(terminal, data, tree)
    }

    fn visit_comma_terminal(
        &mut self,
        terminal: swon_parol::ast::Comma,
        data: swon_parol::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_comma_terminal_super(terminal, data, tree)
    }

    fn visit_esc_terminal(
        &mut self,
        terminal: swon_parol::ast::Esc,
        data: swon_parol::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_esc_terminal_super(terminal, data, tree)
    }

    fn visit_text_start_terminal(
        &mut self,
        terminal: swon_parol::ast::TextStart,
        data: swon_parol::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_text_start_terminal_super(terminal, data, tree)
    }

    fn visit_ident_terminal(
        &mut self,
        terminal: swon_parol::ast::Ident,
        data: swon_parol::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_ident_terminal_super(terminal, data, tree)
    }

    fn visit_named_code_block_begin_terminal(
        &mut self,
        terminal: swon_parol::ast::NamedCodeBlockBegin,
        data: swon_parol::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_named_code_block_begin_terminal_super(terminal, data, tree)
    }

    fn visit_code_block_delimiter_terminal(
        &mut self,
        terminal: swon_parol::ast::CodeBlockDelimiter,
        data: swon_parol::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_code_block_delimiter_terminal_super(terminal, data, tree)
    }

    fn visit_code_block_line_terminal(
        &mut self,
        terminal: swon_parol::ast::CodeBlockLine,
        data: swon_parol::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_code_block_line_terminal_super(terminal, data, tree)
    }

    fn visit_non_terminal(
        &mut self,
        id: swon_parol::tree::CstNodeId,
        kind: swon_parol::nodes::NonTerminalKind,
        data: swon_parol::tree::NonTerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_non_terminal_super(id, kind, data, tree)
    }

    fn visit_non_terminal_close(
        &mut self,
        id: swon_parol::tree::CstNodeId,
        kind: swon_parol::nodes::NonTerminalKind,
        data: swon_parol::tree::NonTerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_non_terminal_close_super(id, kind, data, tree)
    }

    fn visit_terminal(
        &mut self,
        id: swon_parol::tree::CstNodeId,
        kind: swon_parol::nodes::TerminalKind,
        data: swon_parol::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_terminal_super(id, kind, data, tree)
    }
}
