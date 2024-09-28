use std::{collections::{hash_map::{Iter, IterMut}, HashMap}, fmt::Display};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct StateTransitionTable {
    /// State -> Column
    columns: HashMap<(char, char), Vec<char>>,
}

impl Display for StateTransitionTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let transitions = self.columns.iter()
            .map(|((in_state, input), out_state)| {
                format!("({}, {}) -> {:?}", in_state, input, out_state)
            })
            .collect::<Vec<String>>();

        write!(f, "{{\n\t{}\n}}", transitions.join("\n\t"))
    }
}

impl From<Vec<((char, char), Vec<char>)>> for StateTransitionTable {
    fn from(value: Vec<((char, char), Vec<char>)>) -> Self {
        let mut table = StateTransitionTable::new();

        for ((in_state, input), out_state) in value {
            table.insert((in_state, input), out_state);
        }

        table
    }
}

impl StateTransitionTable {
    pub fn new() -> Self {
        Self {
            columns: Default::default()
        }
    }

    pub fn insert(&mut self, key: (char, char), value: Vec<char>) -> Option<Vec<char>> {
        self.columns.insert(key, value)
    }

    pub fn get(&self, key: &(char, char)) -> Option<&Vec<char>> {
        self.columns.get(key)
    }

    pub fn get_mut(&mut self, key: &(char, char)) -> Option<&mut Vec<char>> {
        self.columns.get_mut(key)
    }

    pub fn iter(&self) -> Iter<'_, (char, char), Vec<char>> {
        self.columns.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, (char, char), Vec<char>> {
        self.columns.iter_mut()
    }

    pub fn remove(&mut self, key: &(char, char)) -> Option<Vec<char>> {
        self.columns.remove(key)
    }

    pub fn remove_entry(&mut self, key: &(char, char)) -> Option<((char, char), Vec<char>)> {
        self.columns.remove_entry(key)
    }

    pub fn len(&self) -> usize {
        self.columns.len()
    }
}