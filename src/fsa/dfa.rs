use std::{collections::HashMap, fmt::Display, hash::Hash};

use crate::grammar::Grammar;

use super::{Nfa, FiniteAutomataError, StateTransitionTable};

/// Детерминированный конечный автомат принимает вид
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
pub struct Dfa<State: Eq + Hash, Input> {
    pub states: Vec<State>,
    pub inputs: Vec<Input>,
    pub transitions: StateTransitionTable,
    pub starting_states: Vec<State>,
    pub closing_states: Vec<State>,

    pub state_combo_to_state_map: HashMap<Vec<State>, State>,
    pub state_to_state_combo_map: HashMap<State, Vec<State>>,
}

impl<State: Copy + Eq + Hash, Input: Copy> Display for Dfa<State, Input> 
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

impl TryFrom<Grammar> for Dfa<char, char> {
    type Error = FiniteAutomataError;

    fn try_from(grammar: Grammar) -> Result<Self, Self::Error> {
        Nfa::<char, char>::try_from(grammar)
            .and_then(|nfa| Ok(nfa.to_deterministic()))
    }
}

impl Dfa<char, char> {
    pub fn new(
        states: Vec<char>,
        inputs: Vec<char>,
        transitions: StateTransitionTable,
        starting_states: Vec<char>,
        closing_states: Vec<char>,
        state_combo_to_state_map: HashMap<Vec<char>, char>,
        state_to_state_combo_map: HashMap<char, Vec<char>>,
    ) -> Result<Self, FiniteAutomataError> {
        // validate maps, transitions, starting/closing states

        Ok(Self {
            states,
            inputs,
            transitions,
            starting_states,
            closing_states,

            state_combo_to_state_map,
            state_to_state_combo_map
        })
    }

    pub fn to_non_deterministic(mut self) -> Nfa<char, char> {
        let states_to_remove = self.states.iter()
            .filter(|state| self.state_to_state_combo_map.contains_key(state))
            .cloned()
            .collect::<Vec<char>>();

        states_to_remove.iter()
            .for_each(|state_to_remove| {
                //remove from states vec
                self.states.remove(
                self.states.iter()
                        .position(|state| state == state_to_remove)
                        .expect("Unreachable state should exist in self.states at this point")
                );

                // remove transitions
                self.inputs.iter()
                    .for_each(|input| {
                        self.transitions.remove(&(*state_to_remove, *input));
                    });

                // convert states to underlying combos
                for (_, out_state) in self.transitions.iter_mut() {
                    if out_state[0] == *state_to_remove {
                        *out_state = self.state_to_state_combo_map.get(state_to_remove).cloned().unwrap();
                    }
                }

                // remove from closing states
                if self.closing_states.contains(state_to_remove) {
                    self.closing_states.remove(
                        self.closing_states.iter()
                            .position(|state| state == state_to_remove)
                            .expect("Unreachable state should exist in self.closing_states at this point")
                    );
                }
            });

        Nfa {
            states: self.states,
            inputs: self.inputs,
            transitions: self.transitions,
            starting_states: self.starting_states,
            closing_states: self.closing_states
        }
    }

    pub(crate) fn has_unreachable_states(&self) -> bool {
        let mut reachable_states = vec![];

        self.starting_states.iter()
            .for_each(|state| reachable_states.push(*state));

        let mut temp_vec = reachable_states.clone();

        while let Some(state) = temp_vec.pop() {
            let vec = self.inputs.iter()
                .map(|input| self.transitions.get(&(state, *input)).cloned().unwrap_or(vec![]))
                .flatten()
                .collect::<Vec<char>>();

            vec.iter()
                .for_each(|state| {
                    if !reachable_states.contains(state) {
                        temp_vec.push(*state);
                        reachable_states.push(*state);
                    }
                });
        }

        reachable_states.len() != self.states.len()
    }

    pub(crate) fn remove_unreachable_states(&mut self) {
        let mut reachable_states = vec![];

        self.starting_states.iter()
            .for_each(|state| reachable_states.push(*state));

        let mut temp_vec = reachable_states.clone();

        while let Some(state) = temp_vec.pop() {
            let vec = self.inputs.iter()
                .map(|input| {
                    self.transitions.get(&(state, *input))
                        .cloned()
                        .unwrap_or(vec![])
                })
                .flatten()
                .collect::<Vec<char>>();

            vec.iter()
                .for_each(|state| {
                    if !reachable_states.contains(state) {
                        temp_vec.push(*state);
                        reachable_states.push(*state);
                    }
                });
        }

        let unreachable_states = self.states.iter()
            .filter(|state| !reachable_states.contains(state.to_owned()))
            .cloned()
            .collect::<Vec<char>>();

        unreachable_states.iter().for_each(|unreachable_state| {
            // Убрать функции переходов
            let transitions_to_remove = self.transitions.iter()
                .filter(|(
                    (in_state, _), 
                    out_state
                )| unreachable_state == in_state || out_state.contains(unreachable_state))
                .map(|(left_hand, _)| left_hand)
                .cloned()
                .collect::<Vec<(char, char)>>();
            
            transitions_to_remove.iter()
                .for_each(|left_hand| { self.transitions.remove(left_hand); });
    
            // Убрать из множества состояний
            self.states.remove(
                self.states.iter()
                    .position(|state| state == unreachable_state)
                    .expect("Unreachable state should exist in self.states at this point")
                );

            // Убрать из множества конечных состояний, если таковые имеются
            if self.closing_states.contains(unreachable_state) {
                self.closing_states.remove(
                    self.closing_states.iter()
                        .position(|state| state == unreachable_state)
                        .expect("Unreachable state should exist in self.closing_states at this point")
                );
            }
        });
    }

    pub(crate) fn remove_redundant_states(&mut self) {
        if self.has_unreachable_states() { return; }

        let mut list = vec![];

        let mut state_to_list_idx = HashMap::<char, usize>::new();
        let mut state_to_transitions = HashMap::<(char, char), &Vec<char>>::new();

        // Лист не заканчивающих состояний
        list.push(self.states.iter()
            .filter(|state| !self.closing_states.contains(state))
            .map(|state| { 
                state_to_list_idx.insert(*state, list.len()); 

                self.transitions.iter()
                    .filter(|((in_state, _), _)| in_state == state)
                    .for_each(|((_, input), out_state)| {
                        state_to_transitions.insert((*state, *input), out_state);
                    });

                *state
            })
            .collect::<Vec<char>>());

        // Лист заканчивающих состояний
        list.push(self.states.iter()
            .filter(|state| self.closing_states.contains(state))
            .map(|state| { 
                state_to_list_idx.insert(*state, list.len()); 

                self.transitions.iter()
                    .filter(|((in_state, _), _)| in_state == state)
                    .for_each(|((_, input), out_state)| {
                        state_to_transitions.insert((*state, *input), out_state);
                    });

                *state
            })
            .collect::<Vec<char>>());

        let is_equivalent = |
            s1: char, 
            s2: char, 
            state_to_transitions: &HashMap<(char, char), &Vec<char>>, 
            state_to_list_idx: &HashMap<char, usize>
        | -> bool {
            self.inputs.iter().all(|input| {
                let s1_transitions = state_to_transitions.get(&(s1, *input));
                let s2_transitions = state_to_transitions.get(&(s2, *input));

                match (s1_transitions, s2_transitions) {
                    (Some(s1_transitions), Some(s2_transitions)) => {
                        let [s1_out, s2_out] = [s1_transitions[0], s2_transitions[0]];
                        let s1_out_idx = state_to_list_idx.get(&s1_out);
                        let s2_out_idx = state_to_list_idx.get(&s2_out);

                        if !((s1_out == s2_out) || (s1_out_idx.unwrap() == s2_out_idx.unwrap())) {
                            return false;
                        }
                    },
                    (None, None) => {},
                    _ => {
                        return false;
                    }
                }

                true
            })
        };

        loop {
            let mut new_list: Vec<Vec<char>> = vec![];

            for sublist in &list {
                if sublist.len() == 1 {
                    new_list.push(sublist.to_vec());
                    continue;
                }

                let mut sublist_windows = sublist.windows(2);
                while let Some(states) = sublist_windows.next() {
                    let [s1, s2] = [states[0], states[1]];

                    if is_equivalent(s1, s2, &state_to_transitions, &state_to_list_idx) {
                        match new_list.iter().position(|sublist| sublist.contains(&s1)) {
                            Some(s1_idx) => {
                                match new_list.iter().position(|sublist| sublist.contains(&s2)) {
                                    Some(s2_idx) => {
                                        new_list.remove(s2_idx);
                                    },
                                    None => {}
                                }

                                new_list[s1_idx].push(s2)
                            },
                            None => {
                                new_list.push(vec![s1, s2]);
                            }
                        }
                    } else {
                        let mut is_s1_used = false;
                        let mut is_s2_used = false;

                        new_list.iter_mut()
                            .filter(|new_sublist| {
                                !(new_sublist.contains(&s1) || new_sublist.contains(&s2))
                            })
                            .for_each(|sublist| {
                                if !sublist.contains(&s1) && !is_s1_used &&
                                    is_equivalent(sublist[0], s1, &state_to_transitions, &state_to_list_idx) 
                                {
                                    sublist.push(s1);
                                    is_s1_used = true;
                                } else if !sublist.contains(&s2) && !is_s2_used &&
                                    is_equivalent(sublist[0], s2, &state_to_transitions, &state_to_list_idx) 
                                {
                                    sublist.push(s2);
                                    is_s2_used = true;
                                }
                            });

                        if !is_s1_used {
                            match new_list.iter().position(|sublist| sublist.contains(&s1)) {
                                Some(_) => {},
                                None => {
                                    new_list.push(vec![s1]);
                                }
                            }
                        }

                        if !is_s2_used {
                            match new_list.iter().position(|sublist| sublist.contains(&s2)) {
                                Some(_) => {},
                                None => {
                                    new_list.push(vec![s2]);
                                }
                            }
                        }
                    }
                }
            }

            if new_list == list {
                break;
            } else {
                list = new_list.clone();

                state_to_list_idx.clear();

                for (idx, sublist) in list.iter().enumerate() {
                    for state in sublist {
                        state_to_list_idx.insert(*state, idx); 
                    }
                }
            }
        }

        let mut new_states = vec![];
        let mut new_starting_states = vec![];
        let mut new_closing_states = vec![];

        list.iter().for_each(|sublist| {
            if sublist.len() > 1 {
                let new_state = ('A'..='Z').into_iter()
                    .filter(|ch| !new_states.contains(ch) && !self.states.contains(ch))
                    .next()
                    .expect("Not enough capital letters for states");

                new_states.push(new_state);

                for state in sublist {
                    match self.closing_states.iter().position(|closing_state| closing_state == state) {
                        Some(_) => {
                            if !new_closing_states.contains(&new_state) {
                                new_closing_states.push(new_state);
                            }
                        },
                        _ => {}
                    };

                    match self.starting_states.iter().position(|closing_state| closing_state == state) {
                        Some(_) => {
                            if !new_starting_states.contains(&new_state) {
                                new_starting_states.push(new_state);
                            }
                        },
                        _ => {}
                    };
                }

                self.state_combo_to_state_map.insert(sublist.to_vec(), new_state);
                self.state_to_state_combo_map.insert(new_state, sublist.to_vec());
            } else {
                new_states.push(sublist[0]);

                if self.starting_states.contains(&sublist[0]) {
                    new_starting_states.push(sublist[0]);
                }

                if self.closing_states.contains(&sublist[0]) {
                    new_closing_states.push(sublist[0]);
                }
            }
        });
        
        // Изменить таблицу переходов
        let mut table = StateTransitionTable::new();

        // Создать новую таблицу
        self.transitions.iter()
            .for_each(|((in_state, input), out_state)| {
                let new_in_state = match self.state_combo_to_state_map.iter()
                    .filter(|(combo, _)| combo.contains(in_state)).nth(0)
                {
                    Some((_, state)) => state,
                    None => in_state
                };

                let new_out_state = match self.state_combo_to_state_map.iter()
                    .filter(|(combo, _)| combo.contains(&out_state[0])).nth(0)
                {
                    Some((_, state)) => state,
                    None => &out_state[0]
                };

                table.insert((*new_in_state, *input), vec![*new_out_state]);
            });

        self.transitions = table;
        self.states = new_states;
        self.starting_states = new_starting_states;
        self.closing_states = new_closing_states;
    }

    pub fn minify(&mut self) {
        if self.has_unreachable_states() {
            self.remove_unreachable_states();
        }

        self.remove_redundant_states();
    }
}