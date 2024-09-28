#[macro_export]
macro_rules! rule {
    ($a:literal -> $($b:literal)|+) => {
        crate::grammar::Rule { 
            input: $a.chars().collect(), 
            variants: vec![$($b.chars().collect()),+] 
        }
    }
}

#[macro_export]
macro_rules! rules {
    ($($a:literal -> $($b:literal)|+),*) => {
        vec![$(crate::rule! { $a -> $($b)|* }),*]
    }
}

#[macro_export]
macro_rules! generate {
    ({$($non_term:literal),*}, {$($term:literal),*}, $rules:ident, $start:literal) => {
        crate::grammar::Grammar::new(vec![$($non_term),*], vec![$($term),*], $start, $rules)
    };
    ($non_terms:ident, $terms:ident, $rules:ident, $start:literal) => {
        crate::grammar::Grammar::new($non_terms, $terms, $start, $rules)
    };
    ({$($non_term:literal),*}, {$($term:literal),*}, {$($a:literal -> $($b:literal)|+),*}, $start:literal) => {
        crate::grammar::Grammar::new(vec![$($non_term),*], vec![$($term),*], $start, vec![$(crate::rule! { $a -> $($b)|* }),*])
    };
}