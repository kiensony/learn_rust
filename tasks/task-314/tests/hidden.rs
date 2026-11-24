use task_314::*;

#[test]
fn edge_cases() {
    assert_eq!(prefix_before_final_mark_colon(String::from("")), "");
    assert_eq!(prefix_before_final_mark_colon(String::from(":")), "");
    assert_eq!(prefix_before_final_mark_colon(String::from("cat")), "");
    assert_eq!(
        prefix_before_final_mark_colon(String::from("::one:猫:ab")),
        "::one:猫"
    );
    assert_eq!(prefix_before_final_mark_colon(String::from("aa:bb")), "aa");
    assert_eq!(prefix_before_final_mark_colon(String::from("ok:🐈")), "ok");
}

#[test]
fn submission_marker() {
    assert_eq!(task_314::TRACK_PROOF, 0x52540000 | 314);
}
