mod dpda;
// pub use dpda::Dpda;
mod npda;
pub use npda::Npda;

use std::fmt::{Debug, Display};

#[derive(Debug)]
pub enum PushdownAutomatonError {
    InvalidGrammarType
}

// state, symbol, stack
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct TransitionCondition(pub u32, pub char, pub Option<char>);

impl Display for TransitionCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {:?})", self.0, self.1, self.2)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct TransitionConditionRef<'a>(pub &'a u32, pub char, pub Option<&'a char>);

impl<'a, 'b> hashbrown::Equivalent<TransitionCondition> for TransitionConditionRef<'a> {
    fn equivalent(&self, key: &TransitionCondition) -> bool {
        self.0 == &key.0 && self.1 == key.1 && self.2 == key.2.as_ref()
    }
}
 
// new_state, action
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct TransitionAction(u32, Action);

impl Display for TransitionAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

/// Функции перехода
/// ```text
/// F(q0, t, w0) -> (q1, w1), где
/// 
///     q0 ∈ Q;
/// 
///     t ∈ (T ∪ {ε});
/// 
///     w0 ∈ N;
/// 
///     q1 ∈ Q;
/// 
///     w1 ∈ N*,
/// ```
pub type Transitions = hashbrown::HashMap<
    TransitionCondition,
    Vec<TransitionAction>
>;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Action {
    Push(Vec<char>),
    Pop,
    PopAndPush(Vec<char>)
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Push(state) => write!(f, "{:?}", state),
            Action::Pop => write!(f, "ε"),
            Action::PopAndPush(state) => write!(f, "{:?}", state)
        }
    }
}

pub enum ActionAfterTransition {
    Leave,
    Pop,
    Invalid
}

#[cfg(test)]
mod test{
    use std::collections::VecDeque;

    use crate::generate;
    use super::Npda;

    #[test]
    fn test_nondeterministic() {
        let grammar = generate!{
            {'+', '(', ')', 'a'},
            {'S', 'A'},
            { 
                "S" -> "S+A" | "A",
                "A" -> "(S)" | "a" 
            },
            'S'
        }.expect("Failed to generate grammar");

        println!("{grammar}");

        let mut automaton = Npda::try_from(grammar)
            .expect("Failed to generate automaton");

        println!("{automaton}");
        println!("{:?}", automaton.transitions);

        let valid = automaton.validate(VecDeque::from(['(', 'a', ')']));
        println!("{valid}");
        println!("{automaton}");
    }

    #[test]
    fn test_deterministic() {
        let grammar = generate!{
            {'+', '(', ')', 'a'},
            {'S', 'A'},
            { 
                "S" -> "S+A" | "A",
                "A" -> "(S)" | "a" 
            },
            'S'
        }.expect("Failed to generate grammar");

        println!("{grammar}");

        // let mut automaton = Dpda::try_from(grammar)
        //     .expect("Failed to generate automaton");

        // println!("{automaton}");
        // println!("{}", automaton.states[0]);

        // println!("{}", automaton.validate_input("(a)"));
    }
}