use std::{collections::HashMap, fmt::Display, hash::Hash};

use crate::grammar::{Grammar, GrammarType, RegularType};

use super::{Dfa, FiniteAutomataError, StateTransitionTable};

/// Недетерминированный конечный автомат принимает вид
/// M = (Q, T, F, H, Z), где 
/// 
///     Q - конечное множество состояний автомата;
/// 
///     T - конечное множество допустимых входных символов;
/// 
///     F - функция переходов, отображающая множество Q x T во множество Q;
/// 
///     H - конечное множество начальных состояний автомата;
/// 
///     Z - множество заключительных состояний автомата Z ⊆ (подмножество) Q.
#[derive(Clone, PartialEq, Eq)]
pub struct Nfa<State: Eq + Hash, Input> {
    pub states: Vec<State>,
    pub inputs: Vec<Input>,
    pub transitions: StateTransitionTable,
    pub starting_states: Vec<State>,
    pub closing_states: Vec<State>,
}

impl<State: Copy + Eq + Hash, Input: Copy> Display for Nfa<State, Input> 
    where String: From<State> + From<Input> 
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let states = self.states.iter()
            .map(|sym| String::from(*sym))
            .collect::<Vec<String>>()
            .join(", ");

        let inputs = self.inputs.iter()
            .map(|sym| String::from(*sym))
            .collect::<Vec<String>>()
            .join(", ");

        let starting_states = self.starting_states.iter()
            .map(|sym| String::from(*sym))
            .collect::<Vec<String>>()
            .join(", ");

        let closing_states = self.closing_states.iter()
            .map(|sym| String::from(*sym))
            .collect::<Vec<String>>()
            .join(", ");

        write!(f, "M = {{ {{{}}}, {{{}}}, F, {{{}}}, {{{}}} }}", states, inputs, starting_states, closing_states)
    }
}

impl TryFrom<Grammar> for Nfa<char, char> {
    type Error = FiniteAutomataError;

    fn try_from(mut grammar: Grammar) -> Result<Self, Self::Error> {
        if grammar.grammar_type != GrammarType::Regular(RegularType::Right) {
            return Err(FiniteAutomataError::InavlidGrammarType);
        }

        let closing_non_terminal = if grammar.non_terminals.contains(&'N') {
            ('A'..='Z').into_iter()
                .filter(|ch| !grammar.non_terminals.contains(ch))
                .next()
                .expect("Not enough capital letters for states")
        } else {
            'N'
        };

        let mut states = grammar.non_terminals;
        states.push(closing_non_terminal);

        let inputs = grammar.terminals;
        let starting_states = vec![grammar.starting_non_terminal];
        let mut closing_states = vec![closing_non_terminal];

        // Добавляем правила A -> bN, если существуют A -> b и нет A -> bB
        grammar.rules.iter_mut()
            .for_each(|rule| {
                let variants = rule.variants.clone();
                // println!("{variants:?}");

                rule.variants.iter_mut()
                    .for_each(|variant| { 
                        if variant.len() == 1 {
                            if variant[0] == 'ε' && rule.input[0] == grammar.starting_non_terminal {
                                closing_states.push(grammar.starting_non_terminal);
                            } else if !variants.iter().filter(|v| v.len() > 1).any(|v| v.starts_with(&variant[..])) {
                                variant.push(closing_non_terminal);
                            }
                        }
                    }
                )
            });

        // Построим таблицу функции переходов
        let mut transitions = StateTransitionTable::new();

        grammar.rules.iter()
            .for_each(|rule| {
                // let repeats = Vec::<(char, Vec<char>)>::new();

                rule.variants.iter()
                    .for_each(|variant| {
                        println!("{:?} -> {:?}, {}, {}", rule.input, variant, variant.len(), variant[0] == 'ε');
                        if variant.len() != 1 && variant[0] != 'ε' {
                            println!("{variant:?}");
                            let (arg, output) = (variant[0], variant[1]);
            
                            let vec = match transitions.get_mut(&(rule.input[0], arg)) {
                                Some(vec) => vec,
                                None => {
                                    transitions.insert((rule.input[0], arg), vec![]);
    
                                    transitions.get_mut(&(rule.input[0], arg)).unwrap()
                                }
                            };
    
                            vec.push(output);
    
                            if vec.len() != 1 {
                                vec.sort();
                            }
                        }
                    });
            });

        // let automata_type = if transitions.iter()
        //     .any(|(_, state)| state.len() != 1)
        // {
        //     FiniteAutomataType::NonDeterministic
        // } else {
        //     FiniteAutomataType::Deterministic
        // };
        
        Ok(Self {
            states,
            inputs,
            transitions,
            starting_states,
            closing_states,
        })
    }
}

impl Nfa<char, char> {
    pub fn new(
        states: Vec<char>, 
        inputs: Vec<char>, 
        transitions: StateTransitionTable, 
        starting_states: Vec<char>, 
        closing_states: Vec<char>
    ) -> Result<Self, FiniteAutomataError> {
        // check for invalid starting states, closing states, transitions

        Ok(Self {
            states,
            inputs,
            transitions,
            starting_states,
            closing_states
        })
    }

    pub fn to_deterministic(mut self) -> Dfa<char, char> {
        let mut state_combo_to_state_map = HashMap::new();
        let mut state_to_state_combo_map = HashMap::new();

        let mut states_to_process = Vec::<Vec<char>>::new();

        // Обработать существующие недетерминированные состояния
        self.transitions.iter_mut()
            .filter(|(_, state)| state.len() != 1)
            .for_each(|(_, state)| {
                if state.len() > 0 {
                    if !state_combo_to_state_map.contains_key(&state.to_vec()) {
                        let new_state = ('A'..='Z').into_iter()
                            .filter(|ch| !self.states.contains(ch))
                            .next()
                            .expect("Not enough capital letters for states");
    
                        // println!("!!! {state:?} to {new_state}");
    
                        if self.closing_states.iter()
                            .any(|closing_state| 
                                state.contains(closing_state) && !self.closing_states.contains(&new_state)
                            )
                        {
                            self.closing_states.push(new_state);
                        }
    
                        state_combo_to_state_map.insert(state.to_vec(), new_state);
                        state_to_state_combo_map.insert(new_state, state.to_vec());
    
                        self.states.push(new_state);
    
                        states_to_process.push(state.to_vec());
    
                        *state = vec![new_state];
                    } else {
                        *state = vec![state_combo_to_state_map.get(&state.to_vec()).unwrap().clone()];
                    }
                }
            });

        while let Some(state) = states_to_process.pop() {
            let mut column = self.inputs.iter()
                .map(|input| {
                    let mut out_state = state.iter()
                        .map(|sub_state| {
                            // println!("match ({sub_state}, {input})");
                            let state = match self.transitions.get(&(*sub_state, *input)) {
                                Some(state) => state.clone(),
                                None => vec![]
                            };

                            state.iter().map(|sub_state| {
                                state_to_state_combo_map.get(sub_state).cloned().unwrap_or(vec![*sub_state])
                            })
                            .flatten()
                            .collect::<Vec<char>>()
                        })
                        .flatten()
                        .collect::<Vec<char>>();

                    out_state.sort();
                    out_state.dedup();

                    (*input, out_state)
                })
                .filter(|(_, state)| state.len() > 0)
                .collect::<Vec<(char, Vec<char>)>>();

            // println!("column {column:?} for {state:?}");

            // at this point should be known
            let associated_state = state_combo_to_state_map.get(&state).unwrap().clone();

            column.iter_mut()
                .for_each(|(input, state)| {
                    if state.len() != 1 {
                        // println!("state: {:?}", state);
                        // println!("F = {}", self.transitions);
                        // println!("assocs: {:?}", self.association_map);
                        // println!("states: {:?}", self.states);
                        if state.len() > 0 {
                            if !state_combo_to_state_map.contains_key(&state.to_vec()) {
                                let new_state = ('A'..='Z').into_iter()
                                    .filter(|ch| !self.states.contains(ch))
                                    .next()
                                    .expect("Not enough capital letters for states");
    
                                // println!("!!! {state:?} to {new_state}");
    
                                if self.closing_states.iter()
                                    .any(|closing_state| 
                                        state.contains(closing_state) && !self.closing_states.contains(&new_state)
                                    )
                                {
                                    self.closing_states.push(new_state);
                                }
    
                                state_combo_to_state_map.insert(state.to_vec(), new_state);
                                state_to_state_combo_map.insert(new_state, state.to_vec());
    
                                self.states.push(new_state);
    
                                // println!("push: {state:?}");
                                states_to_process.push(state.to_vec());
    
                                *state = vec![new_state];
                            } else {
                                // println!("set to {state:?}");
                                *state = vec![state_combo_to_state_map.get(&state.to_vec()).unwrap().clone()];
                            }
                        }
                    }

                    self.transitions.insert((associated_state, *input), state.clone());
                });

            // println!("{:?}", self.state_combo_to_state_map);
        }

        Dfa {
            states: self.states,
            inputs: self.inputs,
            transitions: self.transitions,
            starting_states: self.starting_states,
            closing_states: self.closing_states,
            state_combo_to_state_map,
            state_to_state_combo_map
        }
    }
}