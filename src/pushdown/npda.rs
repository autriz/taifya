use std::{cmp::Ordering, collections::{HashMap, HashSet, VecDeque}, fmt::{Debug, Display}, hash::Hash};

use crate::{grammar::{Grammar, GrammarType}, pushdown::ActionAfterTransition};

use super::{Action, PushdownAutomatonError, TransitionAction, TransitionCondition, TransitionConditionRef, Transitions};

/// Автомат с магазинной памятью имеет вид
/// ```text
/// M = (Q, T, N, F, q0, N0, Z), где
/// 
///     Q - конечное множество состояний автомата;
/// 
///     T - конечный входной алфавит;
/// 
///     N - конечный магазинный алфавит;
/// 
///     F - магазинная функция, отображающая множество (Q x (T ∪ {ε} ) x N)
///     во множество всех подмножеств множества Q x N*, т.е.
///     
///             F: (Q x (T ∪ {ε} ) x N) -> P(Q x N*);
/// 
///     q0 - начальное состояние автомата, q0 ∈ Q;
/// 
///     N0 - начальный символ магазина, N0 ∈ N;
/// 
///     Z - множество заключительных состояний автомата, Z ⊆ Q
/// ```
/// Преобразован в
/// ```text
/// TODO: изменить
/// { Q, T, N, q0, N0, Z }, где
/// 
///     Q - массив состояний с принадлежащими им функциями перехода;
///     
///     T - конечный входной алфавит;
/// 
///     N - конечный магазинный алфавит;
/// 
///     q0 - начальное состояние автомата, q0 ∈ Q;
/// 
///     N0 - начальный символ магазина, N0 ∈ N;
/// 
///     Z - множество заключительных состояний автомата, Z ⊆ Q
/// ```
#[derive(Debug)]
pub struct Npda {
    pub start_state: u32,
    pub start_stack: char,
    pub final_states: Vec<u32>,
    pub transitions: Transitions,

    stack: VecDeque<char>
}

impl Display for Npda {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "M = {{ q{}, {{{}}} F }}",
            self.start_state,
            self.final_states.iter().map(|ch| format!("q{ch}")).collect::<Vec<String>>().join(", ")
        )
    }
}

impl TryFrom<Grammar> for Npda {
    type Error = PushdownAutomatonError;

    fn try_from(grammar: Grammar) -> Result<Self, Self::Error> {
        if grammar.grammar_type != GrammarType::ContextFree {
            return Err(PushdownAutomatonError::InvalidGrammarType)
        }

        let start_stack = grammar.starting_non_terminal;
        let inputs = grammar.terminals.clone();
        let mut transitions = hashbrown::HashMap::new();

        // fill table
        for rule in grammar.rules {
            for variant in rule.variants {
                let cond = TransitionCondition(0, 'ε', Some(rule.input[0]));
                let action  = TransitionAction(0, Action::PopAndPush(variant));

                let transition = match transitions.get_mut(&cond) {
                    Some(transition) => transition,
                    None => {
                        let _ = transitions.insert(cond.clone(), vec![]);

                        transitions.get_mut(&cond).unwrap()
                    }
                };

                transition.push(action);
            }
        }

        for input in &inputs {
            let cond = TransitionCondition(0, *input, Some(*input));
            let action  = TransitionAction(0, Action::Pop);
            
            let transition = match transitions.get_mut(&cond) {
                Some(transition) => transition,
                None => {
                    let _ = transitions.insert(cond.clone(), vec![]);

                    transitions.get_mut(&cond).unwrap()
                }
            };

            transition.push(action);
        }

        Ok(Self {
            start_state: 0,
            start_stack: start_stack.clone(),
            final_states: vec![],
            transitions,

            stack: VecDeque::from([start_stack])
        })
    }
}

impl Npda {
    // pub fn validate_input(&self, input: &str) -> bool {
    //     let mut input_vec = input.chars().collect::<Vec<char>>();
    //     let mut stack = vec![self.starting_symbol];
    //     let mut current_state = self.starting_state;

    //     let mut a = 0;

    //     while let Some(ch) = input_vec.get(0) {
    //         let mut functions = match self.states.get(current_state) {
    //             Some(state) => state.rules.iter().filter(|rule| 
    //                 (rule.input == *ch || rule.input == 'ε') 
    //                 && rule.symbols.iter()
    //                     .enumerate()
    //                     .all(|(i, sym)| 
    //                         stack.get(i)
    //                             .is_some_and(|ch| ch == sym)
    //                     )
    //             ).collect::<Vec<&StateRule>>(),
    //             None => return false,
    //         };

    //         println!("{ch}");
    //         println!("{current_state}");
    //         println!("{stack:?}");
    //         println!("{functions:?}");

    //         let function = match functions.len() {
    //             0 => return false,
    //             1 => functions[0],
    //             _ => {
    //                 functions
    //                     .sort_by(|a, b| match (&a.action, &b.action) {
    //                         (RuleAction::Push(a), RuleAction::Push(b)) => {
    //                             let a_count = a.iter()
    //                                 .enumerate()
    //                                 .map(|(i,v)| 
    //                                     if v == input_vec.get(i).unwrap_or(&'\0') { 1 } else { 0 }
    //                                 )
    //                                 .sum::<usize>();

    //                             let b_count = b.iter()
    //                                 .enumerate()
    //                                 .map(|(i,v)| 
    //                                     if v == input_vec.get(i).unwrap_or(&'\0') { 1 } else { 0 }
    //                                 )
    //                                 .sum::<usize>();

    //                             if a_count > b_count {
    //                                 Ordering::Less
    //                             } else if a_count < b_count {
    //                                 Ordering::Greater
    //                             } else {
    //                                 a.len().cmp(&b.len())
    //                             }
    //                         },
    //                         _ => std::cmp::Ordering::Equal,
    //                     });

    //                 println!("a");
    //                 println!("{functions:?}");

    //                 functions[0]
    //             },
    //         };

    //         println!("{function:?}");

    //         if function.input != 'ε' {
    //             input_vec.remove(0);
    //         }

    //         match &function.action {
    //             RuleAction::Push(values) => {
    //                 for _ in &function.symbols {
    //                     let _ = stack.remove(0);
    //                 }

    //                 values.iter().rev().for_each(|value| stack.insert(0, *value));
    //             },
    //             RuleAction::Pop => {
    //                 for _ in &function.symbols {
    //                     let _ = stack.remove(0);
    //                 }
    //             }
    //         }

    //         a += 1;
    //     }

    //     if input_vec.len() != 0 || stack.len() != 0 {
    //         false
    //     } else {
    //         true
    //     }
    // }

    pub fn transition(&mut self, input_symbol: char) -> ActionAfterTransition {
        let stack_top = self.stack.get(0);
        let start_state = self.start_state.clone();
        let mut key = TransitionCondition(start_state, input_symbol, stack_top.copied());

        println!("ref: {key:?}");

        let actions = match self.transitions.get(&key) {
            Some(actions) => actions,
            None => {
                key = TransitionCondition(start_state, 'ε', stack_top.copied());

                match self.transitions.get(&key) {
                    Some(actions) => actions,
                    None => return ActionAfterTransition::Invalid,
                }
            },
        };

        println!("actions: {actions:?}");

        // how to separate them?

        let action = match actions.len() {
            0 => return ActionAfterTransition::Invalid,
            1 => actions[0],
            _ => {
                actions
                    .sort_by(|a, b| match (&a.1, &b.1) {
                        (Action::Push(a), Action::Push(b)) => {
                            let has_start_a = 0;

                            if a_count > b_count {
                                Ordering::Less
                            } else if a_count < b_count {
                                Ordering::Greater
                            } else {
                                a.len().cmp(&b.len())
                            }
                        },
                        _ => std::cmp::Ordering::Equal,
                    });

                println!("a");
                println!("{actions:?}");

                actions[0]
            },
        };

        match action {
            TransitionAction(next_state, Action::Push(state)) => {
                state.iter().rev().for_each(|sym| self.stack.push_front(sym.clone()));
                self.start_state = next_state.clone();
            },
            TransitionAction(next_state, Action::Pop) => {
                self.start_state = next_state.clone();
            },
            TransitionAction(next_state, Action::PopAndPush(state)) => {
                state.iter().rev().for_each(|sym| self.stack.push_front(sym.clone()));
                self.start_state = next_state.clone();
            }
        }

        let action  = match &key.1 {
            'ε' => ActionAfterTransition::Leave,
            _ => ActionAfterTransition::Pop
        };
        
        action
    }

    pub fn validate(&mut self, mut input: VecDeque<char>) -> bool {
        self.reset_stack();

        while let Some(symbol) = input.get(0) {
            match self.transition(*symbol) {
                ActionAfterTransition::Leave => {},
                ActionAfterTransition::Pop => {
                    let _ = input.pop_front().expect("Input should not be empty");
                },
                ActionAfterTransition::Invalid => return false
            }
        }

        if input.len() != 0 || self.stack.len() != 0 {
            false
        } else {
            true
        }
    }

    pub fn get_state(&self) -> &u32 {
        &self.start_state
    }

    pub fn get_stack(&self) -> &VecDeque<char> {
        &self.stack
    }

    fn reset_stack(&mut self) {
        self.stack.clear();
        self.stack.push_front(self.start_stack.clone());
    }
}