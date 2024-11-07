fn main() {
    parol::build::Builder::with_explicit_output_dir("crates/swon-parol")
        .grammar_file("crates/swon-parol/swon.par")
        .parser_output_file("src/parser.rs")
        .actions_output_file("src/grammar_trait.rs")
        .expanded_grammar_output_file("swon-expanded.par")
        .user_type_name("Grammar")
        .user_trait_module_name("grammar")
        .enable_auto_generation()
        .range()
        .minimize_boxed_types()
        // 2 for trailing comma
        .max_lookahead(1)
        .unwrap()
        .generate_parser()
        .unwrap();
}
