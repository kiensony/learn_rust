use task_001::{greet, TRACK_PROOF};

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52535400 | 1);
}

#[test]
fn trim_spaces() {
    assert_eq!(greet("  Ada  "), "Hello, Ada!");
}

#[test]
fn only_whitespace_is_stranger() {
    assert_eq!(greet(" \t  "), "Hello, stranger!");
}

#[test]
fn inner_spaces_kept() {
    assert_eq!(greet("  Ada Lovelace "), "Hello, Ada Lovelace!");
}
