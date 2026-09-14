use std::collections::BTreeMap;
use task_008::{count_words, TRACK_PROOF};

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52535400 | 8);
}

#[test]
fn multi_space_and_tabs() {
    let mut want = BTreeMap::new();
    want.insert("one".into(), 1);
    want.insert("two".into(), 1);
    assert_eq!(count_words("  one\t\ttwo  "), want);
}

#[test]
fn empty() {
    assert!(count_words("   \n\t ").is_empty());
}

#[test]
fn punctuation_stays() {
    let mut want = BTreeMap::new();
    want.insert("ok,".into(), 1);
    want.insert("ok".into(), 1);
    assert_eq!(count_words("OK, ok"), want);
}
