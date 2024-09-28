#[macro_export]
macro_rules! transitions {
    ($($in_state: literal, $input: literal -> $($out_state: literal),+);*) => {
        crate::fsa::StateTransitionTable::from(vec![$((($in_state, $input), vec![$($out_state),*])),*])
    };
}