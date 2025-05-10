use swon_parol::parol_runtime::ParolError;
use swon_parol::tree::CstBuilder;
use swon_parol::{TreeConstruct, parser};
use swon_tree::Cst;

pub enum ParseResult {
    Ok(Cst),
    ErrWithCst { cst: Cst, error: ParolError },
}

/// Parse a document and return a CST
pub fn parse_document(text: &str) -> ParseResult {
    let mut actions = swon_parol::grammar::Grammar::new();
    let mut tree_builder = CstBuilder::new();

    // Parse the document and capture any error
    let parse_result = parser::parse_into(text, &mut tree_builder, "document.swon", &mut actions);

    let cst = tree_builder.build().expect("TreeConstruction never fails");
    // Handle the result
    match parse_result {
        Ok(()) => ParseResult::Ok(cst),
        Err(err) => ParseResult::ErrWithCst { cst, error: err },
    }
}
