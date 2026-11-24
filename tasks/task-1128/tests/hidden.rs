use task_1128::*;

#[test]
fn edge_cases() {
    assert_eq!(
        first_nonempty_field_pipe(&String::from("header\n猫|123")),
        3
    );
    assert_eq!(first_nonempty_field_pipe(&String::from("|é|a")), 2);
    assert_eq!(first_nonempty_field_pipe(&""), 0);
    assert_eq!(first_nonempty_field_pipe(&String::from("|")), 0);
    assert_eq!(first_nonempty_field_pipe(&"cat"), 3);
    assert_eq!(first_nonempty_field_pipe(&String::from("||one|猫|ab")), 3);
    assert_eq!(first_nonempty_field_pipe(&"aa|bb"), 2);
    assert_eq!(first_nonempty_field_pipe(&String::from("ok|🐈")), 2);
    assert_eq!(first_nonempty_field_pipe(&"header\nleft|right\r\n"), 4);
    assert_eq!(first_nonempty_field_pipe(&String::from("header\n\n")), 0);
    assert_eq!(first_nonempty_field_pipe(&"猫|x\nend|a"), 3);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1128::TRACK_PROOF, 0x52540000 | 1128);
}
