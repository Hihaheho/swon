use thiserror::Error;

use crate::{
    Cst, CstConstructError, CstNode, NodeKind,
    node_kind::TerminalKind,
    tree::{CstNodeId, DynamicTokenId, TerminalData},
    visitor::{CstHandleSuper as _, CstVisitor},
};

pub struct FormatVisitor<'f, 't> {
    input: &'t str,
    f: &'f mut dyn std::fmt::Write,
}

impl<'f, 't> FormatVisitor<'f, 't> {
    pub fn new(input: &'t str, f: &'f mut dyn std::fmt::Write) -> Self {
        Self { input, f }
    }
}

#[derive(Error, Debug)]
pub enum FormatVisitorError {
    #[error(transparent)]
    FmtError(#[from] std::fmt::Error),
    #[error(transparent)]
    ViewConstructionError(#[from] CstConstructError),
    #[error("Dynamic token not found: {id:?}")]
    DynamicTokenNotFound { id: DynamicTokenId },
}

impl CstVisitor for FormatVisitor<'_, '_> {
    type Error = FormatVisitorError;

    fn then_construct_error(
        &mut self,
        node_data: Option<CstNode>,
        parent: CstNodeId,
        kind: NodeKind,
        error: CstConstructError,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        eprintln!("Syntax error: {} expected {:?}", error, kind);
        self.recover_error(node_data, parent, kind, tree)
    }

    fn visit_terminal(
        &mut self,
        _id: CstNodeId,
        _kind: TerminalKind,
        terminal: TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        match terminal {
            TerminalData::Input(input_span) => {
                write!(
                    self.f,
                    "{}",
                    &self.input[input_span.start as usize..input_span.end as usize]
                )?;
            }
            TerminalData::Dynamic(id) => {
                let str = tree
                    .dynamic_token(id)
                    .ok_or(FormatVisitorError::DynamicTokenNotFound { id })?;
                write!(self.f, "{}", str)?;
            }
        }
        Ok(())
    }
}

pub struct InspectVisitor<'f, 't> {
    input: &'t str,
    indent: usize,
    f: &'f mut dyn std::fmt::Write,
}

impl<'f, 't> InspectVisitor<'f, 't> {
    pub fn new(input: &'t str, f: &'f mut dyn std::fmt::Write) -> Self {
        Self {
            input,
            f,
            indent: 0,
        }
    }
}

impl CstVisitor for InspectVisitor<'_, '_> {
    type Error = FormatVisitorError;
    fn then_construct_error(
        &mut self,
        node_data: Option<CstNode>,
        parent: CstNodeId,
        kind: NodeKind,
        error: CstConstructError,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        eprintln!("Syntax error: {}", error);
        self.recover_error(node_data, parent, kind, tree)
    }
    fn visit_terminal(
        &mut self,
        _id: CstNodeId,
        kind: crate::node_kind::TerminalKind,
        data: crate::tree::TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        match data {
            TerminalData::Input(input_span) => writeln!(
                self.f,
                "{}{} ({:?})",
                " ".repeat(self.indent),
                &self.input[input_span.start as usize..input_span.end as usize]
                    .replace("\n", "\\n")
                    .replace("\t", "\\t")
                    .replace(" ", "_"),
                kind,
            )?,
            TerminalData::Dynamic(token_id) => writeln!(
                self.f,
                "{}{:?} ({:?})",
                " ".repeat(self.indent),
                tree.dynamic_token(token_id)
                    .ok_or(FormatVisitorError::DynamicTokenNotFound { id: token_id })?,
                kind
            )?,
        }
        Ok(())
    }
    fn visit_non_terminal(
        &mut self,
        _id: CstNodeId,
        kind: crate::node_kind::NonTerminalKind,
        _data: crate::tree::NonTerminalData,
        _tree: &Cst,
    ) -> Result<(), Self::Error> {
        writeln!(self.f, "{}{:?}", " ".repeat(self.indent), kind)?;
        self.indent += 2;
        Ok(())
    }
    fn visit_non_terminal_close(
        &mut self,
        _id: CstNodeId,
        _kind: crate::node_kind::NonTerminalKind,
        _data: crate::tree::NonTerminalData,
        _tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.indent -= 2;
        Ok(())
    }
}
