use task_1107::*;

#[test]
fn edge_cases() {
    assert_eq!(prefix_before_final_mark_colon(&""), 0);
    assert_eq!(prefix_before_final_mark_colon(&String::from(":")), 0);
    assert_eq!(prefix_before_final_mark_colon(&"cat"), 0);
    assert_eq!(
        prefix_before_final_mark_colon(&String::from("::one:猫:ab")),
        9
    );
    assert_eq!(prefix_before_final_mark_colon(&"aa:bb"), 2);
    assert_eq!(prefix_before_final_mark_colon(&String::from("ok:🐈")), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1107::TRACK_PROOF, 0x52540000 | 1107);
}
