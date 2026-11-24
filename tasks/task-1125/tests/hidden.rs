use task_1125::*;

#[test]
fn edge_cases() {
    assert_eq!(marks_removed_at_both_ends_pipe(&"hi|kit"), 6);
    assert_eq!(marks_removed_at_both_ends_pipe(&String::from("|é|a")), 4);
    assert_eq!(marks_removed_at_both_ends_pipe(&""), 0);
    assert_eq!(marks_removed_at_both_ends_pipe(&String::from("|")), 0);
    assert_eq!(marks_removed_at_both_ends_pipe(&"cat"), 3);
    assert_eq!(
        marks_removed_at_both_ends_pipe(&String::from("||one|猫|ab")),
        10
    );
    assert_eq!(marks_removed_at_both_ends_pipe(&"aa|bb"), 5);
    assert_eq!(marks_removed_at_both_ends_pipe(&String::from("ok|🐈")), 7);
    assert_eq!(
        marks_removed_at_both_ends_pipe(&"header\nleft|right\r\n"),
        10
    );
    assert_eq!(
        marks_removed_at_both_ends_pipe(&String::from("header\n\n")),
        0
    );
    assert_eq!(marks_removed_at_both_ends_pipe(&"猫|x\nend|a"), 5);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1125::TRACK_PROOF, 0x52540000 | 1125);
}
