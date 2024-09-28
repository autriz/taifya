mod state_transition_table;
pub use state_transition_table::StateTransitionTable;

mod macros;

mod nfa;
pub use nfa::Nfa;

mod dfa;
pub use dfa::Dfa;

use std::fmt::Display;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum FSAType {
    #[default]
    NonDeterministic,
    Deterministic
}

impl Display for FSAType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_str = match self {
            FSAType::Deterministic => "Детерминированный конечный автомат (ДКА)",
            FSAType::NonDeterministic => "Недетерминированный конечный автомат (НКА)" 
        };

        write!(f, "{}", type_str)
    }
}

#[derive(Debug)]
pub enum FiniteAutomataError {
    // Означает, что тип грамматики не подходит.
    InavlidGrammarType
}

#[cfg(test)]
mod test {
    use crate::{fsa::{Dfa, Nfa}, generate, grammar::{GrammarType, RegularType}, rule, transitions};

    use super::StateTransitionTable;

    #[test]
    fn test_automata() {
        let grammar = generate!{
            {'a', 'b'},
            {'S', 'A', 'B'},
            {
                "S" -> "aB" | "aA",
                "B" -> "bB" | "a",
                "A" -> "aA" | "b"
            },
            'S'
        }.expect("Failed to generate grammar");

        println!("{}", grammar);
        println!("{}", grammar.grammar_type);

        assert_eq!(grammar.grammar_type, GrammarType::Regular(RegularType::Right), "Expected right aligned regular grammar, got {}", grammar.grammar_type);

        let nfa: Nfa<char, char> = grammar.try_into().expect("Failed to generate finite automata");

        println!("\n{}", nfa);
        println!("{}", nfa.transitions);

        let dfa = nfa.to_deterministic();

        println!("\n{}", dfa);
        println!("{}", dfa.transitions);

        let nfa = dfa.to_non_deterministic();

        println!("\n{}", nfa);
        println!("{}", nfa.transitions);
    }

    #[test]
    fn test_remove_unreachable_states() {
        let transitions = transitions!{
            'A','a' -> 'B';
            'A','b' -> 'C';
            'B','b' -> 'D';
            'C','b' -> 'E';
            'D','a' -> 'C';
            'D','b' -> 'E';
            'E','a' -> 'B';
            'E','b' -> 'D';
            'F','a' -> 'D';
            'F','b' -> 'G';
            'G','a' -> 'F';
            'G','b' -> 'E'
        };

        let nfa = Nfa::new(
            vec!['A', 'B', 'C', 'D', 'E', 'F', 'G'], 
            vec!['a', 'b'], 
            transitions,
            vec!['A'],
            vec!['D', 'E']
        ).unwrap();

        let mut dfa = nfa.to_deterministic();

        println!("{}", dfa);
        println!("{}", dfa.transitions);
        
        dfa.remove_unreachable_states();
        
        println!("{}", dfa);
        println!("{}", dfa.transitions);

        assert_eq!(dfa.states, vec!['A', 'B', 'C', 'D', 'E'], "Something went wrong with algorithm");
    }

    #[test]
    fn test_remove_redundant_states() {
        let transitions = transitions!{
            'A','a' -> 'B';
            'A','b' -> 'C';
            'B','b' -> 'D';
            'C','b' -> 'E';
            'D','a' -> 'C';
            'D','b' -> 'E';
            'E','a' -> 'B';
            'E','b' -> 'D'
        };

        let mut dfa = Dfa::new(
            vec!['A', 'B', 'C', 'D', 'E'], 
            vec!['a', 'b'], 
            transitions,
            vec!['A'],
            vec!['D', 'E'],
            Default::default(),
            Default::default()
        ).unwrap();

        println!("{}", dfa);
        println!("{}", dfa.transitions);
        
        println!("has unreachable states: {}", dfa.has_unreachable_states());

        dfa.remove_redundant_states();

        println!("{}", dfa);
        println!("{}", dfa.transitions);

        assert_eq!(dfa.states, vec!['A', 'F', 'G'], "Something went wrong with algorithm");
    }

    #[test]
    fn test_transition() {
        let rules = vec![
            rule! { "S" -> "aB" | "aA" },
            rule! { "B" -> "bB" | "aN" },
            rule! { "A" -> "aA" | "bN" }
        ];

        let mut transitions = StateTransitionTable::new();

        rules.iter()
            .for_each(|rule| {
                rule.variants.iter()
                    .for_each(|variant| {
                        let (arg, output) = (variant[0], variant[1]);
        
                        let vec = match transitions.get_mut(&(rule.input[0], arg)) {
                            Some(vec) => vec,
                            None => {
                                transitions.insert((rule.input[0], arg), vec![]);
        
                                transitions.get_mut(&(rule.input[0], arg)).unwrap()
                            }
                        };
        
                        vec.push(output);
                    });
            });

        println!("{:?}", transitions);
    }
}