use task_361::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_marks_removed_colon(&String::from("")), 0);
    assert_eq!(closing_marks_removed_colon(&String::from(":")), 0);
    assert_eq!(closing_marks_removed_colon(&String::from("cat")), 3);
    assert_eq!(
        closing_marks_removed_colon(&String::from("::one:猫:ab")),
        12
    );
    assert_eq!(closing_marks_removed_colon(&String::from("aa:bb")), 5);
    assert_eq!(closing_marks_removed_colon(&String::from("ok:🐈")), 7);
}

#[test]
fn submission_marker() {
    assert_eq!(task_361::TRACK_PROOF, 0x52540000 | 361);
}
