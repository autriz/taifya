#[test]
fn three() {
    use crate::{fsa::Dfa, transitions};

    let transitions = transitions!{
        'S','(' -> 'M';
        'K','1' -> 'M';
        'L','0' -> 'K';
        'Q','0' -> 'R';
        'T','1' -> 'Q';
        'T',')' -> 'U';
        'M','0' -> 'R';
        'M','1' -> 'R';
        'R',')' -> 'U';
        'R','&' -> 'L';
        'R','~' -> 'O';
        'N','1' -> 'M';
        'O','0' -> 'N';
        'P','1' -> 'R';
        'F','0' -> 'P';
        'F',')' -> 'U'
    };

    let mut dfa = Dfa::new(
        vec!['K', 'L', 'Q', 'T', 'S', 'M', 'R', 'U', 'N', 'O', 'P', 'F'],
        vec!['0', '1', '(', ')', '~', '&'],
        transitions,
        vec!['S'],
        vec!['U'],
        Default::default(),
        Default::default()
    ).unwrap();

    println!("{}", dfa);
    println!("{}", dfa.transitions);

    dfa.remove_unreachable_states();

    println!("{}", dfa);
    println!("{}", dfa.transitions);

    let mut states = dfa.states.clone();
    states.sort();

    let mut expected_states = vec!['S', 'M', 'R', 'U', 'L', 'K', 'O', 'N'];
    expected_states.sort();

    assert_eq!(states, expected_states, "Incorrect state array");

    dfa.remove_redundant_states();

    println!("{}", dfa);
    println!("{}", dfa.transitions);
}