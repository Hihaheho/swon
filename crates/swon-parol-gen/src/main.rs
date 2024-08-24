fn main() {
    parol::build::Builder::with_explicit_output_dir("crates/swon-parol")
        .grammar_file("crates/swon-parol/swon.par")
        .parser_output_file("src/parser.rs")
        .actions_output_file("src/grammar_trait.rs")
        .expanded_grammar_output_file("swon-expanded.par")
        .enable_auto_generation()
        .range()
        .minimize_boxed_types()
        .max_lookahead(5)
        .unwrap()
        .generate_parser()
        .unwrap();
}
