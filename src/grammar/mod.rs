mod macros;

use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum RegularType {
    /// Регулярная грамматика, выровненная влево, имеющая правило вывода вида:
    /// 
    /// A -> Ba | a, где a ∈ Vᴛ, A,B ∈ Vɴ.
    Left,
    /// Регулярная грамматика, выровненная вправо, имеющая правило вывода вида:
    /// 
    /// A -> aB | a, где a ∈ Vᴛ, A,B ∈ Vɴ.
    Right
}

#[derive(Debug, PartialEq, Eq)]
pub enum GrammarType {
    /// Грамматика, не имеющая ограничения на её правила вывода, кроме тех, которые указаны в определении грамматики.
    Type0,
    /// Контекстно-зависимая (КЗ) грамматика, если каждое правило вывода из множества Р
    /// имеет вид:
    /// 
    /// ϕAψ -> ϕaψ, где 
    ///     a ∈ (Vᴛ ∪ Vɴ)+, 
    ///     A ∈ Vɴ, 
    ///     ϕ,ψ ∈ (Vᴛ ∪ Vɴ)*.
    /// 
    /// То есть в каждом правиле нетерминал А в контексте ϕ и ψ заменяется на непустую цепочку a в том же контексте.
    ContextDependent,
    /// Контекстно-свободная (КС) грамматика, правила которой имеют вид:
    /// 
    /// A -> b, где A ∈ Aɴ и b ∈ V*
    ContextFree,
    /// Регулярная грамматика (Р-грамматика).
    Regular(RegularType),
}

impl Display for GrammarType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            GrammarType::Type0 => "Тип 0",
            GrammarType::ContextDependent => "Тип 1 (КЗ-грамматика)",
            GrammarType::ContextFree => "Тип 2 (КС-грамматика)",
            GrammarType::Regular(RegularType::Left) => "Тип 3 (Р-грамматика, выровненная влево)",
            GrammarType::Regular(RegularType::Right) => "Тип 3 (Р-грамматика, выровненная вправо)"
        };

        write!(f, "{}", text)
    }
}

#[derive(Debug)]
pub enum GrammarError {
    // Означает, что в терминальных и нетерминальных символах имеются пересекающиеся символы.
    OverlappingSymbols,
    // Означает, что в множестве нетерминальных символов нет символа S.
    MissingStartingNonTerminalSymbol,
    // Означает, что правило, определённое для грамматики, не подходит.
    InvalidRule
}

#[derive(Debug)]
pub struct Rule {
    pub input: Vec<char>,
    pub variants: Vec<Vec<char>>,
}

impl Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let input = String::from_iter(&self.input);

        let variants = self.variants.iter()
            .map(|variant| String::from_iter(variant).into())
            .collect::<Vec<String>>()
            .join(" | ");

        write!(f, "{} -> {}", input, variants)
    }
}

pub struct Grammar {
    pub terminals: Vec<char>,
    pub non_terminals: Vec<char>,
    pub rules: Vec<Rule>,
    pub starting_non_terminal: char,
    pub grammar_type: GrammarType
}

impl Display for Grammar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rules = self.rules.iter().map(|rule| format!("{{{rule}}}")).collect::<Vec<String>>().join(", ");

        let terminals = self.terminals.iter()
            .map(|sym| String::from(*sym))
            .collect::<Vec<String>>()
            .join(", ");

        let non_terminals = self.non_terminals.iter()
            .map(|sym| String::from(*sym))
            .collect::<Vec<String>>()
            .join(", ");

        write!(f, "G = {{ {{{}}}, {{{}}}, {{{}}}, {} }}", terminals, non_terminals, &rules, self.starting_non_terminal)
    }
}

impl Grammar {
    const EMPTY_SEQUENCE: char = 'ε';

    pub fn new(
        terminals: Vec<char>, 
        non_terminals: Vec<char>, 
        starting_non_terminal: char,
        rules: Vec<Rule>
    ) -> Result<Self, GrammarError> {
        if terminals.iter()
            .any(|sym| non_terminals.contains(sym)) 
        {
            return Err(GrammarError::OverlappingSymbols);
        }

        if !non_terminals.contains(&starting_non_terminal) {
            return Err(GrammarError::MissingStartingNonTerminalSymbol);
        }

        if !rules.iter()
            .all(|rule| {
                let valid_input = rule.input.iter()
                    .all(|sym| 
                        terminals.contains(sym) || non_terminals.contains(sym)
                    );
                
                let valid_variants = rule.variants.iter()
                    .all(|variant| 
                        variant.iter()
                            .all(|sym| {
                                let is_terminal = terminals.contains(sym);
                                let is_non_terminal = non_terminals.contains(sym);
                                let is_empty = sym == &Self::EMPTY_SEQUENCE;
                                let is_operation = ['+', '-', '*', '/'].contains(sym);

                                is_terminal || is_non_terminal || is_empty || is_operation
                            })
                        );

                valid_input && valid_variants
        }) {
            return Err(GrammarError::InvalidRule);
        }

        let grammar_type = Grammar::get_type(&terminals, &non_terminals, &rules);

        Ok(Self {
            terminals,
            non_terminals,
            rules,
            starting_non_terminal,
            grammar_type
        })
    }

    pub fn is_grammar_language_exists(&self) -> bool {
        if self.grammar_type != GrammarType::ContextFree { return false; }

        let list = self.get_non_terminals_with_terminal_strings();

        list.contains(&self.starting_non_terminal)
    }

    pub fn remove_non_ending_non_terminals(&mut self) {
        if self.grammar_type != GrammarType::ContextFree { return; }

        let new_non_terminals = self.get_non_terminals_with_terminal_strings();

        let mut new_rules = vec![];

        self.rules.iter().for_each(|rule| {
            if new_non_terminals.contains(&rule.input[0]) {
                let variants = rule.variants.iter()
                    .filter(|variant| 
                        variant.iter()
                            .all(|ch| 
                                self.terminals.contains(ch) || 
                                *ch == Self::EMPTY_SEQUENCE
                            )
                        )
                    .cloned()
                    .collect::<Vec<Vec<char>>>();

                new_rules.push(
                    Rule { input: vec![rule.input[0]], variants }
                );
            }
        });

        self.non_terminals = new_non_terminals;
        self.rules = new_rules;     
    } 

    pub fn remove_unreachable_symbols(&mut self) {
        let mut non_terminals = vec![self.starting_non_terminal];
        let mut terminals = vec![];

        loop {
            let mut new_non_terminals = non_terminals.clone();
            let mut new_terminals = terminals.clone();

            self.rules.iter().for_each(|rule| {
                if new_non_terminals.contains(&rule.input[0]) {
                    rule.variants.iter().for_each(|variant| {
                        variant.iter().for_each(|ch| {
                            if self.terminals.contains(ch) && !new_terminals.contains(ch) {
                                new_terminals.push(*ch);
                            }

                            if self.non_terminals.contains(ch) && !new_non_terminals.contains(ch) {
                                new_non_terminals.push(*ch);
                            }
                        });
                    });
                }
            });

            if new_non_terminals == non_terminals && new_terminals == terminals {
                break;
            } else {
                non_terminals = new_non_terminals.clone();
                terminals = new_terminals.clone();
            }
        }

        let mut rules = vec![];

        self.rules.iter().for_each(|rule| {
            if non_terminals.contains(&rule.input[0]) {
                let variants = rule.variants.iter()
                    .filter(|variant| 
                        variant.iter()
                            .all(|ch| 
                                self.terminals.contains(ch) || 
                                *ch == Self::EMPTY_SEQUENCE
                            )
                        )
                    .cloned()
                    .collect::<Vec<Vec<char>>>();

                rules.push(
                    Rule { input: vec![rule.input[0]], variants }
                );
            }
        });

        self.terminals = terminals;
        self.non_terminals = non_terminals;
        self.rules = rules;
    }

    fn remove_empty_rules(&mut self) {

    }

    pub fn make_equivalent(&mut self) {
        if self.grammar_type != GrammarType::ContextFree { return; }


    }

    fn get_type(
        terminals: &Vec<char>, 
        non_terminals: &Vec<char>, 
        rules: &Vec<Rule>
    ) -> GrammarType {
        let mut grammar_type = GrammarType::Type0;

        // check for type 1
        if rules.iter()
            .all(|rule| {
                rule.variants.iter().all(|variant| rule.input.len() <= variant.len())
            })
        {
            grammar_type = GrammarType::ContextDependent;
        } else {
            return grammar_type;
        }

        if rules.iter()
            .all(|rule| rule.input.len() == 1)
        {
            grammar_type = GrammarType::ContextFree;
        } else {
            return grammar_type;
        }

        let mut regular_type = None;

        if rules.iter()
            .all(|rule| {
                rule.variants.iter().all(|variant| {
                    let is_left_aligned = non_terminals.iter().any(|sym| variant.starts_with(&[*sym]));
                    let is_right_aligned = non_terminals.iter().any(|sym| variant.ends_with(&[*sym]));
                    let is_terminated = variant.len() == 1 && terminals.contains(&variant[0]);
                    let is_empty = variant.len() == 1 && variant[0] == Self::EMPTY_SEQUENCE;

                    match (is_left_aligned, is_right_aligned) {
                        (true, false) => regular_type = Some(RegularType::Left),
                        (false, true) => regular_type = Some(RegularType::Right),
                        _ => { 
                            if !is_terminated && !is_empty { 
                                return false;
                            } 
                        }
                    }

                    is_left_aligned || is_right_aligned || is_terminated || is_empty
                })
            })
        {
            grammar_type = GrammarType::Regular(regular_type.unwrap());
        }

        grammar_type
    }

    fn get_non_terminals_with_terminal_strings(&self) -> Vec<char> {
        let mut list = vec![];

        loop {
            let mut new_list = list.clone();

            for non_terminal in &self.non_terminals {
                if self.rules.iter()
                    .any(|rule| 
                        rule.input.contains(non_terminal) && 
                        rule.variants.iter()
                            .any(|variant|
                                variant.iter().all(|ch| 
                                    self.terminals.contains(ch) || 
                                    new_list.contains(ch) || 
                                    *ch == Self::EMPTY_SEQUENCE
                                )
                            )
                        ) &&
                    !new_list.contains(non_terminal)
                {
                    new_list.push(*non_terminal);
                }
            }

            if new_list == list {
                break;
            } else {
                list = new_list.clone();
            }
        }

        list
    }
}

#[cfg(test)]
mod test {
    use crate::{
        generate, grammar::{Grammar, GrammarType, RegularType}, rule
    };

    #[test]
    fn test_grammar_types() {
        let grammar = generate!{
            {'a', 'b', 'c', 'd'},
            {'A', 'B', 'S'},
            { 
                "A" -> "aB" | "ε" 
            },
            'S'
        }.expect("Failed to generate grammar");

        println!("{}", grammar.grammar_type);

        assert_eq!(grammar.grammar_type, GrammarType::Regular(RegularType::Right), "Expected regular grammar type, got: {}", grammar.grammar_type);

        let terminals = vec!['a', 'b', 'c', 'd'];
        let non_terminals = vec!['A', 'B', 'S'];
        let rules = vec![
            rule! { "A" -> "bBc" },
        ];

        let grammar = Grammar::new(terminals, non_terminals, 'S', rules)
            .expect("Failed to generate grammar");

        println!("{}", grammar.grammar_type);

        assert_eq!(grammar.grammar_type, GrammarType::ContextFree, "Expected context-free grammar type, got: {}", grammar.grammar_type);

        let terminals = vec!['a', 'b', 'c', 'd'];
        let non_terminals = vec!['A', 'B', 'C', 'S'];
        let rules = vec![
            rule! { "CB" -> "BC" },
        ];

        let grammar = Grammar::new(terminals, non_terminals, 'S', rules)
            .expect("Failed to generate grammar");

        println!("{}", grammar.grammar_type);

        assert_eq!(grammar.grammar_type, GrammarType::ContextDependent, "Expected context-dependent grammar type, got: {}", grammar.grammar_type);

        let terminals = vec!['a', 'b', 'c', 'd'];
        let non_terminals = vec!['A', 'B', 'C', 'S'];
        let rules = vec![
            rule! { "AB" -> "bBA" },
            rule! { "bCB" -> "ε" },
        ];

        let grammar = Grammar::new(terminals, non_terminals, 'S', rules)
            .expect("Failed to generate grammar");

        println!("{}", grammar.grammar_type);

        assert_eq!(grammar.grammar_type, GrammarType::Type0, "Expected type 0 grammar, got: {}", grammar.grammar_type);
    }

    #[test]
    fn test_is_grammar_language_exists() {
        let grammar = generate!{
            {'0', '1'},
            {'S', 'A', 'B'},
            {
                "S" -> "AB",
                "A" -> "0A" | "0",
                "B" -> "1"
            },
            'S'
        }.expect("Failed to generate grammar");

        println!("{}", grammar);
        println!("{}", grammar.grammar_type);

        assert_eq!(grammar.grammar_type, GrammarType::ContextFree, "Expected context-free grammar type, got: {}", grammar.grammar_type);

        println!("is grammar language exists: {}", grammar.is_grammar_language_exists());

        assert!(grammar.is_grammar_language_exists(), "Grammar language should exist for this grammar");
    }

    #[test]
    fn test_remove_non_terminals_without_terminals() {
        let mut grammar = generate!{
            {'a', 'b', 'c'},
            {'S', 'A', 'B', 'C'},
            {
                "S" -> "ab" | "AC",
                "A" -> "AB",
                "B" -> "b",
                "C" -> "cb"
            },
            'S'
        }.expect("Failed to generate grammar");

        println!("{}", grammar);
        println!("{}", grammar.grammar_type);

        grammar.remove_non_ending_non_terminals();

        println!("{}", grammar);
        println!("{}", grammar.grammar_type);

        assert_eq!(grammar.non_terminals, vec!['S', 'B', 'C'], "Invalid non-terminals, got: {:?}", grammar.non_terminals);
    }

    #[test]
    fn test_remove_unreachable_symbols() {
        let mut grammar = generate!{
            {'a', 'b', 'c'},
            {'S', 'B', 'C'},
            {
                "S" -> "ab",
                "B" -> "b",
                "C" -> "cb"
            },
            'S'
        }.expect("Failed to generate grammar");

        println!("{}", grammar);
        println!("{}", grammar.grammar_type);

        grammar.remove_unreachable_symbols();

        println!("{}", grammar);
        println!("{}", grammar.grammar_type);

        assert_eq!(grammar.non_terminals, vec!['S'], "Invalid non-terminals, got: {:?}", grammar.non_terminals);
        assert_eq!(grammar.terminals, vec!['a', 'b'], "Invalid terminals, got: {:?}", grammar.terminals);
    }

    #[test]
    fn test_remove_empty_rules() {
        let mut grammar = generate!{
            {'0', '1'},
            {'S', 'A', 'B'},
            {
                "S" -> "AB",
                "A" -> "0A" | "ε",
                "B" -> "1B" | "ε"
            },
            'S'
        }.expect("Failed to generate grammar");

        grammar.remove_empty_rules();

        println!("{}", grammar);
        println!("{}", grammar.grammar_type);

        assert_eq!(grammar.non_terminals, vec!['S', 'A', 'B', 'C'], "Invalid non-terminals, got: {:?}", grammar.non_terminals);
        assert_eq!(grammar.starting_non_terminal, 'C', "Invalid starting non-terminal, got: {}", grammar.starting_non_terminal);
        assert_eq!(grammar.terminals, vec!['0', '1'], "Invalid terminals, got: {:?}", grammar.terminals);
    }
}