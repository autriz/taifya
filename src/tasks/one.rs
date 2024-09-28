#[test]
fn one() {
    use crate::grammar::GrammarType;

    // Составляющие грамматики вида
    // G = { V_T, V_N, P, S },
    // где V_T = {a, b, c, d},
    //     V_N = {A, B, S}
    //
    // для языка вида
    // L(G) = { (ab)^n * (cd)^m | n,m >= 0 }

    let grammar = crate::generate!{
        {'a', 'b', 'c', 'd'},
        {'A', 'B', 'S'},
        {
            "S" -> "AB" | "A" | "B",
            "A" -> "ab" | "abA" | "ε",
            "B" -> "cd" | "cdB" | "ε"
        },
        'S'
    }.expect("Failed to generate grammar");

    println!("{}", grammar);
    println!("{}", grammar.grammar_type);

    assert_eq!(grammar.grammar_type, GrammarType::ContextFree, "Expected context-free grammar type, got: {}", grammar.grammar_type);
}