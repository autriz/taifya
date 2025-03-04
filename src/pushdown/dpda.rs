// use std::{cmp::Ordering, fmt::Display};

// use crate::grammar::{Grammar, GrammarType};

// use super::{Action, PushdownAutomatonError, TransitionAction, TransitionCondition, TransitionConditionRef, Transitions};

// pub struct Dpda {
//     pub states: Vec<State<String>>,
//     pub inputs: Vec<char>,
//     pub alphabet: Vec<char>,

//     pub starting_state: usize,
//     pub starting_symbol: char,

//     pub ending_states: Vec<char>,
// }

// impl TryFrom<Grammar> for Dpda {
//     type Error = PushdownAutomatonError;

//     fn try_from(grammar: Grammar) -> Result<Self, Self::Error> {
//         if grammar.grammar_type != GrammarType::ContextFree {
//             return Err(PushdownAutomatonError::InvalidGrammarType)
//         }

//         let inputs = grammar.terminals.clone();
//         let starting_symbol = '#';

//         let alphabet = [grammar.terminals, grammar.non_terminals, vec!['#']].concat();

//         let mut state_rules = vec![];

//         // fill table
//         for rule in grammar.rules {
//             for variant in rule.variants {
//                 state_rules.push(
//                     StateRule {
//                         next_state: 0,
//                         input: 'ε',
//                         symbols: rule.input.clone(),
//                         action: RuleAction::Push(variant)
//                     }
//                 )
//             }
//         }

//         for input in &inputs {
//             state_rules.push(StateRule {
//                 next_state: 0,
//                 input: *input,
//                 symbols: vec!['ε'],
//                 action: RuleAction::Push(vec![*input])
//             });
//         }

//         state_rules.push(StateRule {
//             next_state: 1,
//             input: 'ε',
//             symbols: vec!['S'],
//             action: RuleAction::Pop
//         });

//         let states = vec![
//             State { id: 0, rules: state_rules, state_type: StateType::Normal },
//             State { id: 1, rules: vec![], state_type: StateType::Final }
//         ];

//         Ok(Self {
//             states,
//             inputs,
//             alphabet,
//             starting_state: 0,
//             starting_symbol,
//             ending_states: vec![],
//         })
//     }
// }

// impl Display for Dpda {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "M = {{ {{{}}}, {{{}}}, {{{}}}, F, q{}, {}, {{{}}} }}",
//             self.states.iter().enumerate().map(|(i, _)| format!("q{i}")).collect::<Vec<String>>().join(", "),
//             self.inputs.iter().map(|ch| format!("{ch}")).collect::<Vec<String>>().join(", "),
//             self.alphabet.iter().map(|ch| format!("{ch}")).collect::<Vec<String>>().join(", "),
//             self.starting_state,
//             self.starting_symbol,
//             self.ending_states.iter().map(|ch| format!("{ch}")).collect::<Vec<String>>().join(", ")
//         )
//     }
// }

// impl Dpda {
//     pub fn validate_input(&self, input: &str) -> bool {
//         let mut input_vec = input.chars().collect::<Vec<char>>();
//         let mut stack = vec![self.starting_symbol];
//         let mut current_state = self.starting_state;

//         while let Some(ch) = input_vec.get(0) {
//             let mut functions = match self.states.get(current_state) {
//                 Some(state) => state.rules.iter().filter(|rule| {
//                     let left = rule.input == *ch || rule.input == 'ε';

//                     let right = rule.symbols.iter()
//                         .enumerate()
//                         .all(|(i, sym)| 
//                             stack.get(i)
//                                 .is_some_and(|ch| ch == sym)
//                         );

//                     left && (right || rule.symbols.contains(&'ε'))
//                 }
//                 ).collect::<Vec<&StateRule>>(),
//                 None => return false,
//             };

//             println!("{ch}");
//             println!("{current_state}");
//             println!("{stack:?}");
//             println!("{functions:?}");

//             let function = match functions.len() {
//                 0 => return false,
//                 1 => functions[0],
//                 _ => {
//                     functions
//                         .sort_by(|a, b| match (&a.action, &b.action) {
//                             (RuleAction::Push(a), RuleAction::Push(b)) => {
//                                 let a_count = a.iter()
//                                     .enumerate()
//                                     .map(|(i,v)| 
//                                         if v == input_vec.get(i).unwrap_or(&'\0') { 1 } else { 0 }
//                                     )
//                                     .sum::<usize>();

//                                 let b_count = b.iter()
//                                     .enumerate()
//                                     .map(|(i,v)| 
//                                         if v == input_vec.get(i).unwrap_or(&'\0') { 1 } else { 0 }
//                                     )
//                                     .sum::<usize>();

//                                 if a_count > b_count {
//                                     Ordering::Less
//                                 } else if a_count < b_count {
//                                     Ordering::Greater
//                                 } else {
//                                     a.len().cmp(&b.len())
//                                 }
//                             },
//                             _ => std::cmp::Ordering::Equal,
//                         });

//                     println!("a");
//                     println!("{functions:?}");

//                     functions[0]
//                 },
//             };

//             println!("{function:?}");

//             if function.input != 'ε' {
//                 input_vec.remove(0);
//             }

//             match &function.action {
//                 RuleAction::Push(values) => {
//                     values.iter().for_each(|value| stack.push(*value));
//                 },
//                 RuleAction::Pop => {
//                     for _ in &function.symbols {
//                         if let None =  stack.pop() {
//                             return false;
//                         }
//                     }
//                 }
//             }
//         }

//         println!("after");
//         println!("{current_state}");
//         println!("{stack:?}");

//         if input_vec.len() != 0 || stack.len() != 0 {
//             false
//         } else {
//             true
//         }
//     }
// }