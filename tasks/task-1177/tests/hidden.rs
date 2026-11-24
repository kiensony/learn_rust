use task_1177::*;

#[test]
fn edge_cases() {
    assert_eq!(stored_key(&[], ""), None);
    assert_eq!(stored_key(&[""], ""), Some(""));
    assert_eq!(stored_key(&["λ", "Λ"], "Λ"), Some("Λ"));
}

#[test]
fn submission_marker() {
    assert_eq!(task_1177::TRACK_PROOF, 0x52540000 | 1177);
}
