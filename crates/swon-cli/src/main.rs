use clap::{Args, Parser, Subcommand};
use std::fs;
use swon_parol::TreeConstruct;
use swon_parol::grammar::Grammar;
use swon_parol::parser::parse_into;
use swon_parol::tree::CstBuilder;

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
            let mut tree_builder = CstBuilder::new();
            if let Err(e) = parse_into(&contents, &mut tree_builder, &file, &mut grammar) {
                eprintln!("Error parsing file: {}", e);
            }
            let tree = tree_builder.build().unwrap();
            let mut out = String::new();
            tree.inspect(&contents, &mut out).unwrap();
            println!("{}", out);
        }
    }
}
