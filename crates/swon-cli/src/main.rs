use clap::{Args, Parser, Subcommand};
use std::fs;
use swon_parol::grammar::Grammar;
use swon_parol::parol_runtime::parser::parse_tree_type::{SynTree2, SynTreeTerminal, TerminalData};
use swon_parol::parol_runtime::parser::parser_types::SynTreeFlavor;
use swon_parol::parol_runtime::ParolError;
use swon_parol::parser::parse2;
use swon_parol::syntree::Node;

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
            let tree = match parse2::<SynTree2<Grammar<'_>>>(&contents, &file, &mut grammar) {
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

            for child in tree.children() {
                print_tree(&contents, &child, 0);
            }
        }
    }
}

fn print_tree(input: &str, node: &Node<SynTree2<Grammar<'_>>, SynTreeFlavor>, indent: usize) {
    match node.value() {
        SynTree2::Terminal(t) => match t.data {
            TerminalData::Input(input_span) => println!(
                "{}{} ({:?})",
                " ".repeat(indent),
                &input[input_span.start as usize..input_span.end as usize]
                    .replace("\n", "\\n")
                    .replace("\t", "\\t")
                    .replace(" ", "_"),
                t.kind,
            ),
            TerminalData::Dynamic(_) => unreachable!(),
        },
        SynTree2::NonTerminal(n) => println!("{}{:?}", " ".repeat(indent), n.kind),
    }
    if let SynTree2::Terminal(SynTreeTerminal {
        data: TerminalData::Input(input_span),
        ..
    }) = node.value()
    {
        use swon_parol::syntree::Span;
        assert_eq!(node.span(), Span::new(input_span.start, input_span.end));
    }
    for child in node.children() {
        print_tree(input, &child, indent + 2);
    }
}
