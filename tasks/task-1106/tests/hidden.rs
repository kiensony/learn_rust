use task_1106::*;

#[test]
fn edge_cases() {
    assert_eq!(remainder_after_first_mark_colon(&""), 0);
    assert_eq!(remainder_after_first_mark_colon(&String::from(":")), 0);
    assert_eq!(remainder_after_first_mark_colon(&"cat"), 0);
    assert_eq!(
        remainder_after_first_mark_colon(&String::from("::one:猫:ab")),
        11
    );
    assert_eq!(remainder_after_first_mark_colon(&"aa:bb"), 2);
    assert_eq!(remainder_after_first_mark_colon(&String::from("ok:🐈")), 4);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1106::TRACK_PROOF, 0x52540000 | 1106);
}
