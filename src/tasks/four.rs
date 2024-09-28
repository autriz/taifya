#[test]
fn four() {
    use crate::{generate, grammar::GrammarType};

    let mut grammar = generate!{
        {'+', '-', '*', '/', 'n', 'm', 'h'},
        {'E', 'T', 'F', 'G', 'H'},
        {
            "E" -> "T" | "E+T" | "E-T" | "ε",
            "T" -> "F" | "F*T" | "F/T" | "ε",
            "F" -> "G" | "Fn" | "n",
            "G" -> "Gm",
            "H" -> "Hh" | "h"
        },
        'E'
    }.expect("Failed to generate grammar");

    println!("{}", grammar);
    println!("{}", grammar.grammar_type);

    assert_eq!(grammar.grammar_type, GrammarType::ContextFree, "Expected context-free grammar, got: {}", grammar.grammar_type);

    assert!(grammar.is_grammar_language_exists(), "This grammar should have its grammar language");

    if grammar.is_grammar_language_exists() {
        grammar.remove_non_ending_non_terminals();

        println!("{}", grammar);
        println!("{}", grammar.grammar_type);

        // grammar.remove_unreachable_symbols();

        // println!("{}", grammar);
        // println!("{}", grammar.grammar_type);
    }
}