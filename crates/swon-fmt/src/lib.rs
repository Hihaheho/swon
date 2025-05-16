#[cfg(any(feature = "unformat", test))]
pub mod unformat;

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
}
