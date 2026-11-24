use task_1056::*;

#[test]
fn edge_cases() {
    assert_eq!(marks_removed_at_both_ends_colon(&""), 0);
    assert_eq!(marks_removed_at_both_ends_colon(&String::from(":")), 0);
    assert_eq!(marks_removed_at_both_ends_colon(&"cat"), 3);
    assert_eq!(
        marks_removed_at_both_ends_colon(&String::from("::one:猫:ab")),
        10
    );
    assert_eq!(marks_removed_at_both_ends_colon(&"aa:bb"), 5);
    assert_eq!(marks_removed_at_both_ends_colon(&String::from("ok:🐈")), 7);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1056::TRACK_PROOF, 0x52540000 | 1056);
}
