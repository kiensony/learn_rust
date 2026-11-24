use task_287::*;

#[test]
fn edge_cases() {
    assert_eq!(count_separators_tilde(""), "0");
    assert_eq!(count_separators_tilde("~"), "1");
    assert_eq!(count_separators_tilde("~~"), "2");
    assert_eq!(count_separators_tilde(" café "), "0");
    assert_eq!(count_separators_tilde("a~b"), "1");
    assert_eq!(count_separators_tilde("🐈~猫"), "1");
}

#[test]
fn submission_marker() {
    assert_eq!(task_287::TRACK_PROOF, 0x52540000 | 287);
}
