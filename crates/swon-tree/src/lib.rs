use std::convert::Infallible;

use nodes::{NonTerminalKind, TerminalKind};
use tree::{ConcreteSyntaxTree, ViewConstructionError};

pub mod action;
pub mod ast;
pub mod common_visitors;
pub mod nodes;
pub mod tree;
pub mod value_visitor;
pub mod visitor;

pub type Cst = ConcreteSyntaxTree<TerminalKind, NonTerminalKind>;
pub type CstNode = tree::CstNodeData<TerminalKind, NonTerminalKind>;
pub type CstConstructError<E = Infallible> =
    ViewConstructionError<TerminalKind, NonTerminalKind, E>;
pub type NodeKind = nodes::NodeKind<TerminalKind, NonTerminalKind>;
