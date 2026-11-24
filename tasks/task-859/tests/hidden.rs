use task_859::*;

#[test]
fn edge_cases() {
    assert_eq!(prefix_before_final_mark_colon(""), 2);
    assert_eq!(prefix_before_final_mark_colon(":"), 2);
    assert_eq!(prefix_before_final_mark_colon("cat"), 2);
    assert_eq!(prefix_before_final_mark_colon("::one:猫:ab"), 11);
    assert_eq!(prefix_before_final_mark_colon("aa:bb"), 4);
    assert_eq!(prefix_before_final_mark_colon("ok:🐈"), 4);
}

#[test]
fn submission_marker() {
    assert_eq!(task_859::TRACK_PROOF, 0x52540000 | 859);
}
