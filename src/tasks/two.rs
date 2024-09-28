#[test]
fn two() {
    use crate::{
        grammar::{ GrammarType, RegularType },
        fsa::Nfa,
        generate
    };

    let grammar = generate!{
        {'a', 'b', 'c'},
        {'S', 'A', 'B', 'C'},
        {
            "S" -> "aA" | "bB" | "aC" | "b",
            "A" -> "bA" | "bB" | "c",
            "B" -> "aA" | "cC" | "b",
            "C" -> "bB" | "bC" | "a"
        },
        'S'
    }.expect("Failed to generate grammar");

    println!("{}", grammar);
    println!("{}", grammar.grammar_type);

    assert_eq!(grammar.grammar_type, GrammarType::Regular(RegularType::Right), "Expected right aligned regular grammar type, got: {}", grammar.grammar_type);

    let nfa: Nfa<char, char> = grammar.try_into().expect("Failed to generate finite automata");

    println!("\n{}", nfa);
    println!("{}", nfa.transitions);

    let dfa = nfa.to_deterministic();

    println!("\n{}", dfa);
    println!("{}", dfa.transitions);
    println!("{:?}", dfa.state_combo_to_state_map);
}