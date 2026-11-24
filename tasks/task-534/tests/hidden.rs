use task_534::*;

#[test]
fn edge_cases() {
    assert_eq!(preview("éa", 2), "é");
    assert_eq!(preview("", 100), "");
    assert_eq!(preview("🌱ab", 5), "🌱a");
    assert_eq!(preview("a", usize::MAX), "a");
}

#[test]
fn submission_marker() {
    assert_eq!(task_534::TRACK_PROOF, 0x52540000 | 534);
}
