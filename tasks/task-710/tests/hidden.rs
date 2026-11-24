use task_710::*;

#[test]
fn edge_cases() {
    assert_eq!(prefix_before_final_mark_colon(Some(":é:a")), None);
    assert_eq!(prefix_before_final_mark_colon(Some("")), None);
    assert_eq!(prefix_before_final_mark_colon(Some(":")), None);
    assert_eq!(prefix_before_final_mark_colon(Some("cat")), None);
    assert_eq!(prefix_before_final_mark_colon(Some("::one:猫:ab")), None);
    assert_eq!(prefix_before_final_mark_colon(Some("aa:bb")), None);
    assert_eq!(prefix_before_final_mark_colon(Some("ok:🐈")), None);
    assert_eq!(prefix_before_final_mark_colon(None), None);
    assert_eq!(prefix_before_final_mark_colon(Some(":9")), None);
    assert_eq!(prefix_before_final_mark_colon(Some("3300")), None);
    assert_eq!(prefix_before_final_mark_colon(Some("65536")), None);
}

#[test]
fn submission_marker() {
    assert_eq!(task_710::TRACK_PROOF, 0x52540000 | 710);
}
