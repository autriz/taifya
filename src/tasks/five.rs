#[test]
fn five() {
    use crate::pushdown::Npda;
    use crate::{generate, grammar::GrammarType};

    let grammar = generate!{
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

    let automaton = Npda::try_from(grammar)
        .expect("Failed to generate automaton");

    // println!("{automaton}");
    // for state in &automaton.states {
    //     println!("{}", state);
    // }

    // println!("{}", automaton.validate_input("n-n+nn*n"))
}