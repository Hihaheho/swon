use clap::{Args, Parser, Subcommand};
use std::fs;
use swon_parol::{grammar::Grammar, parser::parse};

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
            let tree = match parse(&contents, &file, &mut grammar) {
                Ok(tree) => tree,
                Err(e) => {
                    eprintln!("Error parsing file: {:?}", e);
                    return;
                }
            };

            println!("{:?}", tree);
        }
    }
}
