use task_1161::*;

#[test]
fn edge_cases() {
    assert_eq!(setting("="), Some(("", "")));
    assert_eq!(setting("none"), None);
    assert_eq!(setting("鍵=値"), Some(("鍵", "値")));
}

#[test]
fn submission_marker() {
    assert_eq!(task_1161::TRACK_PROOF, 0x52540000 | 1161);
}
