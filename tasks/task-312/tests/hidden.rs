use task_312::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_marks_removed_colon(String::from("")), "");
    assert_eq!(closing_marks_removed_colon(String::from(":")), "");
    assert_eq!(closing_marks_removed_colon(String::from("cat")), "cat");
    assert_eq!(
        closing_marks_removed_colon(String::from("::one:猫:ab")),
        "::one:猫:ab"
    );
    assert_eq!(closing_marks_removed_colon(String::from("aa:bb")), "aa:bb");
    assert_eq!(closing_marks_removed_colon(String::from("ok:🐈")), "ok:🐈");
}

#[test]
fn submission_marker() {
    assert_eq!(task_312::TRACK_PROOF, 0x52540000 | 312);
}
