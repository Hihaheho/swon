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
        handle: swon_tree::nodes::ArrayHandle,
        view: swon_tree::nodes::ArrayView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_array_super(view, tree)
    }

    fn visit_array_begin(
        &mut self,
        handle: swon_tree::nodes::ArrayBeginHandle,
        view: swon_tree::nodes::ArrayBeginView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_array_begin_super(view, tree)
    }

    fn visit_array_end(
        &mut self,
        handle: swon_tree::nodes::ArrayEndHandle,
        view: swon_tree::nodes::ArrayEndView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_array_end_super(view, tree)
    }

    fn visit_array_list(
        &mut self,
        handle: swon_tree::nodes::ArrayListHandle,
        view: swon_tree::nodes::ArrayListView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_array_list_super(view, tree)
    }

    fn visit_array_marker(
        &mut self,
        handle: swon_tree::nodes::ArrayMarkerHandle,
        view: swon_tree::nodes::ArrayMarkerView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_array_marker_super(view, tree)
    }

    fn visit_array_marker_opt(
        &mut self,
        handle: swon_tree::nodes::ArrayMarkerOptHandle,
        view: swon_tree::nodes::IntegerHandle,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_array_marker_opt_super(view, tree)
    }

    fn visit_array_opt(
        &mut self,
        handle: swon_tree::nodes::ArrayOptHandle,
        view: swon_tree::nodes::CommaHandle,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_array_opt_super(view, tree)
    }

    fn visit_at(
        &mut self,
        handle: swon_tree::nodes::AtHandle,
        view: swon_tree::nodes::AtView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_at_super(view, tree)
    }

    fn visit_begin(
        &mut self,
        handle: swon_tree::nodes::BeginHandle,
        view: swon_tree::nodes::BeginView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_begin_super(view, tree)
    }

    fn visit_bind(
        &mut self,
        handle: swon_tree::nodes::BindHandle,
        view: swon_tree::nodes::BindView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_bind_super(view, tree)
    }

    fn visit_binding(
        &mut self,
        handle: swon_tree::nodes::BindingHandle,
        view: swon_tree::nodes::BindingView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_binding_super(view, tree)
    }

    fn visit_binding_rhs(
        &mut self,
        handle: swon_tree::nodes::BindingRhsHandle,
        view: swon_tree::nodes::BindingRhsView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_binding_rhs_super(view, tree)
    }

    fn visit_boolean(
        &mut self,
        handle: swon_tree::nodes::BooleanHandle,
        view: swon_tree::nodes::BooleanView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_boolean_super(view, tree)
    }

    fn visit_code(
        &mut self,
        handle: swon_tree::nodes::CodeHandle,
        view: swon_tree::nodes::CodeView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_code_super(view, tree)
    }

    fn visit_code_block(
        &mut self,
        handle: swon_tree::nodes::CodeBlockHandle,
        view: swon_tree::nodes::CodeBlockView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_code_block_super(view, tree)
    }

    fn visit_code_block_delimiter(
        &mut self,
        handle: swon_tree::nodes::CodeBlockDelimiterHandle,
        view: swon_tree::nodes::CodeBlockDelimiterView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_code_block_delimiter_super(view, tree)
    }

    fn visit_code_block_line(
        &mut self,
        handle: swon_tree::nodes::CodeBlockLineHandle,
        view: swon_tree::nodes::CodeBlockLineView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_code_block_line_super(view, tree)
    }

    fn visit_code_block_tail_common(
        &mut self,
        handle: swon_tree::nodes::CodeBlockTailCommonHandle,
        view: swon_tree::nodes::CodeBlockTailCommonView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_code_block_tail_common_super(view, tree)
    }

    fn visit_code_block_tail_common_list(
        &mut self,
        handle: swon_tree::nodes::CodeBlockTailCommonListHandle,
        view: swon_tree::nodes::CodeBlockTailCommonListView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_code_block_tail_common_list_super(view, tree)
    }

    fn visit_code_block_tail_common_opt(
        &mut self,
        handle: swon_tree::nodes::CodeBlockTailCommonOptHandle,
        view: swon_tree::nodes::WsHandle,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_code_block_tail_common_opt_super(view, tree)
    }

    fn visit_comma(
        &mut self,
        handle: swon_tree::nodes::CommaHandle,
        view: swon_tree::nodes::CommaView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_comma_super(view, tree)
    }

    fn visit_continue(
        &mut self,
        handle: swon_tree::nodes::ContinueHandle,
        view: swon_tree::nodes::ContinueView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_continue_super(view, tree)
    }

    fn visit_dot(
        &mut self,
        handle: swon_tree::nodes::DotHandle,
        view: swon_tree::nodes::DotView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_dot_super(view, tree)
    }

    fn visit_end(
        &mut self,
        handle: swon_tree::nodes::EndHandle,
        view: swon_tree::nodes::EndView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_end_super(view, tree)
    }

    fn visit_ext(
        &mut self,
        handle: swon_tree::nodes::ExtHandle,
        view: swon_tree::nodes::ExtView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_ext_super(view, tree)
    }

    fn visit_extension_name_space(
        &mut self,
        handle: swon_tree::nodes::ExtensionNameSpaceHandle,
        view: swon_tree::nodes::ExtensionNameSpaceView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_extension_name_space_super(view, tree)
    }

    fn visit_false(
        &mut self,
        handle: swon_tree::nodes::FalseHandle,
        view: swon_tree::nodes::FalseView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_false_super(view, tree)
    }

    fn visit_hole(
        &mut self,
        handle: swon_tree::nodes::HoleHandle,
        view: swon_tree::nodes::HoleView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_hole_super(view, tree)
    }

    fn visit_ident(
        &mut self,
        handle: swon_tree::nodes::IdentHandle,
        view: swon_tree::nodes::IdentView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_ident_super(view, tree)
    }

    fn visit_in_str(
        &mut self,
        handle: swon_tree::nodes::InStrHandle,
        view: swon_tree::nodes::InStrView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_in_str_super(view, tree)
    }

    fn visit_integer(
        &mut self,
        handle: swon_tree::nodes::IntegerHandle,
        view: swon_tree::nodes::IntegerView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_integer_super(view, tree)
    }

    fn visit_key(
        &mut self,
        handle: swon_tree::nodes::KeyHandle,
        view: swon_tree::nodes::KeyView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_key_super(view, tree)
    }

    fn visit_key_base(
        &mut self,
        handle: swon_tree::nodes::KeyBaseHandle,
        view: swon_tree::nodes::KeyBaseView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_key_base_super(view, tree)
    }

    fn visit_key_opt(
        &mut self,
        handle: swon_tree::nodes::KeyOptHandle,
        view: swon_tree::nodes::ArrayMarkerHandle,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_key_opt_super(view, tree)
    }

    fn visit_keys(
        &mut self,
        handle: swon_tree::nodes::KeysHandle,
        view: swon_tree::nodes::KeysView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_keys_super(view, tree)
    }

    fn visit_keys_list(
        &mut self,
        handle: swon_tree::nodes::KeysListHandle,
        view: swon_tree::nodes::KeysListView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_keys_list_super(view, tree)
    }

    fn visit_named_code(
        &mut self,
        handle: swon_tree::nodes::NamedCodeHandle,
        view: swon_tree::nodes::NamedCodeView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_named_code_super(view, tree)
    }

    fn visit_named_code_block(
        &mut self,
        handle: swon_tree::nodes::NamedCodeBlockHandle,
        view: swon_tree::nodes::NamedCodeBlockView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_named_code_block_super(view, tree)
    }

    fn visit_named_code_block_begin(
        &mut self,
        handle: swon_tree::nodes::NamedCodeBlockBeginHandle,
        view: swon_tree::nodes::NamedCodeBlockBeginView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_named_code_block_begin_super(view, tree)
    }

    fn visit_newline(
        &mut self,
        handle: swon_tree::nodes::NewlineHandle,
        view: swon_tree::nodes::NewlineView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_newline_super(view, tree)
    }

    fn visit_null(
        &mut self,
        handle: swon_tree::nodes::NullHandle,
        view: swon_tree::nodes::NullView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_null_super(view, tree)
    }

    fn visit_object(
        &mut self,
        handle: swon_tree::nodes::ObjectHandle,
        view: swon_tree::nodes::ObjectView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_object_super(view, tree)
    }

    fn visit_object_list(
        &mut self,
        handle: swon_tree::nodes::ObjectListHandle,
        view: swon_tree::nodes::ObjectListView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_object_list_super(view, tree)
    }

    fn visit_object_opt(
        &mut self,
        handle: swon_tree::nodes::ObjectOptHandle,
        view: swon_tree::nodes::CommaHandle,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_object_opt_super(view, tree)
    }

    fn visit_quote(
        &mut self,
        handle: swon_tree::nodes::QuoteHandle,
        view: swon_tree::nodes::QuoteView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_quote_super(view, tree)
    }

    fn visit_section(
        &mut self,
        handle: swon_tree::nodes::SectionHandle,
        view: swon_tree::nodes::SectionView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_section_super(view, tree)
    }

    fn visit_section_binding(
        &mut self,
        handle: swon_tree::nodes::SectionBindingHandle,
        view: swon_tree::nodes::SectionBindingView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_section_binding_super(view, tree)
    }

    fn visit_section_body(
        &mut self,
        handle: swon_tree::nodes::SectionBodyHandle,
        view: swon_tree::nodes::SectionBodyView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_section_body_super(view, tree)
    }

    fn visit_section_body_list(
        &mut self,
        handle: swon_tree::nodes::SectionBodyListHandle,
        view: swon_tree::nodes::SectionBodyListView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_section_body_list_super(view, tree)
    }

    fn visit_str(
        &mut self,
        handle: swon_tree::nodes::StrHandle,
        view: swon_tree::nodes::StrView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_str_super(view, tree)
    }

    fn visit_str_continues(
        &mut self,
        handle: swon_tree::nodes::StrContinuesHandle,
        view: swon_tree::nodes::StrContinuesView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_str_continues_super(view, tree)
    }

    fn visit_str_continues_list(
        &mut self,
        handle: swon_tree::nodes::StrContinuesListHandle,
        view: swon_tree::nodes::StrContinuesListView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_str_continues_list_super(view, tree)
    }

    fn visit_swon(
        &mut self,
        handle: swon_tree::nodes::SwonHandle,
        view: swon_tree::nodes::SwonView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_swon_super(view, tree)
    }

    fn visit_swon_bindings(
        &mut self,
        handle: swon_tree::nodes::SwonBindingsHandle,
        view: swon_tree::nodes::SwonBindingsView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_swon_bindings_super(view, tree)
    }

    fn visit_swon_sections(
        &mut self,
        handle: swon_tree::nodes::SwonSectionsHandle,
        view: swon_tree::nodes::SwonSectionsView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_swon_sections_super(view, tree)
    }

    fn visit_text(
        &mut self,
        handle: swon_tree::nodes::TextHandle,
        view: swon_tree::nodes::TextView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_text_super(view, tree)
    }

    fn visit_text_binding(
        &mut self,
        handle: swon_tree::nodes::TextBindingHandle,
        view: swon_tree::nodes::TextBindingView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_text_binding_super(view, tree)
    }

    fn visit_text_binding_opt(
        &mut self,
        handle: swon_tree::nodes::TextBindingOptHandle,
        view: swon_tree::nodes::WsHandle,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_text_binding_opt_super(view, tree)
    }

    fn visit_text_start(
        &mut self,
        handle: swon_tree::nodes::TextStartHandle,
        view: swon_tree::nodes::TextStartView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_text_start_super(view, tree)
    }

    fn visit_true(
        &mut self,
        handle: swon_tree::nodes::TrueHandle,
        view: swon_tree::nodes::TrueView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_true_super(view, tree)
    }

    fn visit_typed_quote(
        &mut self,
        handle: swon_tree::nodes::TypedQuoteHandle,
        view: swon_tree::nodes::TypedQuoteView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_typed_quote_super(view, tree)
    }

    fn visit_typed_str(
        &mut self,
        handle: swon_tree::nodes::TypedStrHandle,
        view: swon_tree::nodes::TypedStrView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_typed_str_super(view, tree)
    }

    fn visit_value(
        &mut self,
        handle: swon_tree::nodes::ValueHandle,
        view: swon_tree::nodes::ValueView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_value_super(view, tree)
    }

    fn visit_value_binding(
        &mut self,
        handle: swon_tree::nodes::ValueBindingHandle,
        view: swon_tree::nodes::ValueBindingView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_value_binding_super(view, tree)
    }

    fn visit_ws(
        &mut self,
        handle: swon_tree::nodes::WsHandle,
        view: swon_tree::nodes::WsView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_ws_super(view, tree)
    }

    fn visit_root(
        &mut self,
        handle: swon_tree::nodes::RootHandle,
        view: swon_tree::nodes::RootView,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        let _handle = handle;
        self.visit_root_super(view, tree)
    }

    fn visit_new_line_terminal(
        &mut self,
        terminal: swon_tree::nodes::NewLine,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_new_line_terminal_super(terminal, data, tree)
    }

    fn visit_whitespace_terminal(
        &mut self,
        terminal: swon_tree::nodes::Whitespace,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_whitespace_terminal_super(terminal, data, tree)
    }

    fn visit_line_comment_terminal(
        &mut self,
        terminal: swon_tree::nodes::LineComment,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_line_comment_terminal_super(terminal, data, tree)
    }

    fn visit_block_comment_terminal(
        &mut self,
        terminal: swon_tree::nodes::BlockComment,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_block_comment_terminal_super(terminal, data, tree)
    }

    fn visit_integer_terminal(
        &mut self,
        terminal: swon_tree::nodes::Integer,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_integer_terminal_super(terminal, data, tree)
    }

    fn visit_true_terminal(
        &mut self,
        terminal: swon_tree::nodes::True,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_true_terminal_super(terminal, data, tree)
    }

    fn visit_false_terminal(
        &mut self,
        terminal: swon_tree::nodes::False,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_false_terminal_super(terminal, data, tree)
    }

    fn visit_null_terminal(
        &mut self,
        terminal: swon_tree::nodes::Null,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_null_terminal_super(terminal, data, tree)
    }

    fn visit_hole_terminal(
        &mut self,
        terminal: swon_tree::nodes::Hole,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_hole_terminal_super(terminal, data, tree)
    }

    fn visit_quote_terminal(
        &mut self,
        terminal: swon_tree::nodes::Quote,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_quote_terminal_super(terminal, data, tree)
    }

    fn visit_typed_quote_terminal(
        &mut self,
        terminal: swon_tree::nodes::TypedQuote,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_typed_quote_terminal_super(terminal, data, tree)
    }

    fn visit_in_str_terminal(
        &mut self,
        terminal: swon_tree::nodes::InStr,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_in_str_terminal_super(terminal, data, tree)
    }

    fn visit_text_terminal(
        &mut self,
        terminal: swon_tree::nodes::Text,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_text_terminal_super(terminal, data, tree)
    }

    fn visit_named_code_terminal(
        &mut self,
        terminal: swon_tree::nodes::NamedCode,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_named_code_terminal_super(terminal, data, tree)
    }

    fn visit_code_terminal(
        &mut self,
        terminal: swon_tree::nodes::Code,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_code_terminal_super(terminal, data, tree)
    }

    fn visit_newline_terminal(
        &mut self,
        terminal: swon_tree::nodes::Newline,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_newline_terminal_super(terminal, data, tree)
    }

    fn visit_ws_terminal(
        &mut self,
        terminal: swon_tree::nodes::Ws,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_ws_terminal_super(terminal, data, tree)
    }

    fn visit_at_terminal(
        &mut self,
        terminal: swon_tree::nodes::At,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_at_terminal_super(terminal, data, tree)
    }

    fn visit_dollar_terminal(
        &mut self,
        terminal: swon_tree::nodes::Dollar,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_dollar_terminal_super(terminal, data, tree)
    }

    fn visit_dot_terminal(
        &mut self,
        terminal: swon_tree::nodes::Dot,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_dot_terminal_super(terminal, data, tree)
    }

    fn visit_l_brace_terminal(
        &mut self,
        terminal: swon_tree::nodes::LBrace,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_l_brace_terminal_super(terminal, data, tree)
    }

    fn visit_r_brace_terminal(
        &mut self,
        terminal: swon_tree::nodes::RBrace,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_r_brace_terminal_super(terminal, data, tree)
    }

    fn visit_l_bracket_terminal(
        &mut self,
        terminal: swon_tree::nodes::LBracket,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_l_bracket_terminal_super(terminal, data, tree)
    }

    fn visit_r_bracket_terminal(
        &mut self,
        terminal: swon_tree::nodes::RBracket,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_r_bracket_terminal_super(terminal, data, tree)
    }

    fn visit_bind_terminal(
        &mut self,
        terminal: swon_tree::nodes::Bind,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_bind_terminal_super(terminal, data, tree)
    }

    fn visit_comma_terminal(
        &mut self,
        terminal: swon_tree::nodes::Comma,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_comma_terminal_super(terminal, data, tree)
    }

    fn visit_esc_terminal(
        &mut self,
        terminal: swon_tree::nodes::Esc,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_esc_terminal_super(terminal, data, tree)
    }

    fn visit_text_start_terminal(
        &mut self,
        terminal: swon_tree::nodes::TextStart,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_text_start_terminal_super(terminal, data, tree)
    }

    fn visit_ident_terminal(
        &mut self,
        terminal: swon_tree::nodes::Ident,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_ident_terminal_super(terminal, data, tree)
    }

    fn visit_named_code_block_begin_terminal(
        &mut self,
        terminal: swon_tree::nodes::NamedCodeBlockBegin,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_named_code_block_begin_terminal_super(terminal, data, tree)
    }

    fn visit_code_block_delimiter_terminal(
        &mut self,
        terminal: swon_tree::nodes::CodeBlockDelimiter,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_code_block_delimiter_terminal_super(terminal, data, tree)
    }

    fn visit_code_block_line_terminal(
        &mut self,
        terminal: swon_tree::nodes::CodeBlockLine,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_code_block_line_terminal_super(terminal, data, tree)
    }

    fn visit_non_terminal(
        &mut self,
        id: swon_tree::tree::CstNodeId,
        kind: swon_tree::node_kind::NonTerminalKind,
        data: swon_tree::tree::NonTerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_non_terminal_super(id, kind, data, tree)
    }

    fn visit_non_terminal_close(
        &mut self,
        id: swon_tree::tree::CstNodeId,
        kind: swon_tree::node_kind::NonTerminalKind,
        data: swon_tree::tree::NonTerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_non_terminal_close_super(id, kind, data, tree)
    }

    fn visit_terminal(
        &mut self,
        id: swon_tree::tree::CstNodeId,
        kind: swon_tree::node_kind::TerminalKind,
        data: swon_tree::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.visit_terminal_super(id, kind, data, tree)
    }
}
