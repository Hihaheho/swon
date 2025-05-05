use clap::{Args, Parser, Subcommand};
use std::fs;
use swon_parol::grammar::Grammar;
use swon_parol::nodes::{NonTerminalKind, TerminalKind};
use swon_parol::parol_runtime::ParolError;
use swon_parol::parser::parse_into;
use swon_parol::tree::{CstBuilder, CstNodeData};
use petgraph::graph::NodeIndex;

#[derive(Parser)]
#[command(name = "swon", about = "SWON file utilities")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse and display SWON file syntax tree
    Inspect(Inspect),
}

#[derive(Args)]
struct Inspect {
    /// Path to SWON file to inspect
    file: String,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Inspect(Inspect { file }) => {
            let contents = match fs::read_to_string(&file) {
                Ok(contents) => contents,
                Err(e) => {
                    eprintln!("Error reading file: {}", e);
                    return;
                }
            };

            let mut grammar = Grammar::new();
            let tree_builder =
                CstBuilder::<TerminalKind, NonTerminalKind>::new();
            let tree = match parse_into(&contents, tree_builder, &file, &mut grammar) {
                Ok(tree) => tree,
                Err(e) => {
                    match e {
                        ParolError::ParserError(e) => eprintln!("Error parsing file: {:?}", e),
                        ParolError::LexerError(e) => eprintln!("Error lexing file: {:?}", e),
                        ParolError::UserError(e) => eprintln!("Error: {:?}", e),
                    }
                    return;
                }
            };

            if tree.root.is_some() {
                print_tree(&contents, &tree, 0);
            }
        }
    }
}

fn print_tree(
    input: &str,
    tree: &swon_parol::tree::ConcreteSyntaxTree<TerminalKind, NonTerminalKind>,
    indent: usize,
) {
    if let Some(root) = tree.root {
        print_node(input, tree, root, indent);
    }
}

fn print_node(
    input: &str,
    tree: &swon_parol::tree::ConcreteSyntaxTree<TerminalKind, NonTerminalKind>,
    node_idx: petgraph::graph::NodeIndex,
    indent: usize,
) {
    if let Some(node_data) = tree.node_data(node_idx) {
        match node_data {
            CstNodeData::Terminal(kind, input_span) => println!(
                "{}{} ({:?})",
                " ".repeat(indent),
                &input[input_span.start as usize..input_span.end as usize]
                    .replace("\n", "\\n")
                    .replace("\t", "\\t")
                    .replace(" ", "_"),
                kind,
            ),
            CstNodeData::NonTerminal(kind) => println!("{}{:?}", " ".repeat(indent), kind),
            CstNodeData::DynamicToken(token_id) => println!("{}{:?} (dynamic)", " ".repeat(indent), token_id),
        }
    }

    for child in tree.children(node_idx) {
        print_node(input, tree, child, indent + 2);
    }
}
