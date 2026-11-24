use task_383::*;

#[test]
fn edge_cases() {
    assert_eq!(remainder_after_first_mark_pipe(&String::from("hi|kit")), 3);
    assert_eq!(remainder_after_first_mark_pipe(&String::from("|é|a")), 4);
    assert_eq!(remainder_after_first_mark_pipe(&String::from("")), 0);
    assert_eq!(remainder_after_first_mark_pipe(&String::from("|")), 0);
    assert_eq!(remainder_after_first_mark_pipe(&String::from("cat")), 0);
    assert_eq!(
        remainder_after_first_mark_pipe(&String::from("||one|猫|ab")),
        11
    );
    assert_eq!(remainder_after_first_mark_pipe(&String::from("aa|bb")), 2);
    assert_eq!(remainder_after_first_mark_pipe(&String::from("ok|🐈")), 4);
    assert_eq!(
        remainder_after_first_mark_pipe(&String::from("header\nleft|right\r\n")),
        5
    );
    assert_eq!(
        remainder_after_first_mark_pipe(&String::from("header\n\n")),
        0
    );
    assert_eq!(
        remainder_after_first_mark_pipe(&String::from("猫|x\nend|a")),
        1
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_383::TRACK_PROOF, 0x52540000 | 383);
}
