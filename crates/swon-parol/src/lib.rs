// This module is generated by swon-parol-gen.
pub mod grammar;
#[allow(clippy::needless_lifetimes)]
pub mod grammar_trait;
pub mod parser;
pub use parol_runtime;
use parol_runtime::parser::parse_tree_type::GrammarEnums;
pub use parol_runtime::syntree;

impl GrammarEnums for grammar::Grammar<'_> {
    type NonTerminalEnum = grammar_trait::NonTerminalKind;
    type TerminalEnum = grammar_trait::TerminalKind;
}

#[test]
fn test_parse() {
    let mut actions = grammar::Grammar::new();
    let input = r#"
    @ a.b.c
	d = 1 # comment
    e = "aaa"
	"#;
    let tree = parser::parse2::<
        parol_runtime::parser::parse_tree_type::SynTree2<grammar::Grammar<'_>>,
    >(input, "test.swon", &mut actions)
    .unwrap();
    let result = tree.walk().fold(String::new(), |mut acc, node| {
        if !node.has_children() {
            println!("{:?}", node.value());
            let span = node.span();
            acc.push_str(&input[span.start as usize..span.end as usize]);
        }
        acc
    });
    assert_eq!(input, result);
}

// use grammar_trait::*;

// trait Reconstruct {
//     fn reconstruct<W: std::fmt::Write>(&self, writer: &mut W) -> String;
// }

// impl Grammar<'_> {
//     pub fn reconstruct(&self) -> String {
//         let mut writer = String::new();
//         self.swon.reconstruct(&mut writer);
//         writer
//     }
// }

// impl<T> Reconstruct for Option<T>
// where
//     T: Reconstruct,
// {
//     fn reconstruct(&self) -> String {
//         match self {
//             Some(t) => t.reconstruct(),
//             None => String::default(),
//         }
//     }
// }

// impl<T> Reconstruct for Vec<T>
// where
//     T: Reconstruct,
// {
//     fn reconstruct(&self) -> String {
//         self.iter()
//             .map(|t| t.reconstruct())
//             .collect::<Vec<_>>()
//             .join("")
//     }
// }

// impl Reconstruct for Swon<'_> {
//     fn reconstruct(&self) -> String {
//         self.swon_list0
//             .iter()
//             .map(|section| section.section.reconstruct())
//     }
// }

// impl Reconstruct for Section<'_> {
//     fn reconstruct(&self) -> String {
//         todo!()
//     }
// }
