//! `string.hostname` length semantics: at most 253 characters, not counting
//! the optional trailing dot (upstream protovalidate#512 / #513).

use protovalidate_buffa::rules::string::is_hostname;

fn name_of_len(len: usize) -> String {
    // "123456789." repeated, then letters so the final label is not numeric.
    let mut s = "123456789.".repeat(len / 10);
    while s.len() < len {
        s.push('a');
    }
    s
}

#[test]
fn accepts_253_characters() {
    let name = name_of_len(253);
    assert_eq!(name.len(), 253);
    assert!(is_hostname(&name));
}

#[test]
fn accepts_253_characters_plus_trailing_dot() {
    let name = format!("{}.", name_of_len(253));
    assert_eq!(name.len(), 254);
    assert!(is_hostname(&name));
}

#[test]
fn rejects_254_characters() {
    assert!(!is_hostname(&name_of_len(254)));
}

#[test]
fn rejects_empty_and_lone_dot() {
    assert!(!is_hostname(""));
    assert!(!is_hostname("."));
}
