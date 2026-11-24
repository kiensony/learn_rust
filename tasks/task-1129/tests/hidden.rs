use task_1129::*;

#[test]
fn edge_cases() {
    assert_eq!(
        longest_field_with_last_tie_pipe(&String::from("header\n猫|123")),
        3
    );
    assert_eq!(longest_field_with_last_tie_pipe(&"hi|kit"), 3);
    assert_eq!(longest_field_with_last_tie_pipe(&""), 0);
    assert_eq!(longest_field_with_last_tie_pipe(&String::from("|")), 0);
    assert_eq!(longest_field_with_last_tie_pipe(&"cat"), 3);
    assert_eq!(
        longest_field_with_last_tie_pipe(&String::from("||one|猫|ab")),
        3
    );
    assert_eq!(longest_field_with_last_tie_pipe(&"aa|bb"), 2);
    assert_eq!(longest_field_with_last_tie_pipe(&String::from("ok|🐈")), 4);
    assert_eq!(
        longest_field_with_last_tie_pipe(&"header\nleft|right\r\n"),
        5
    );
    assert_eq!(
        longest_field_with_last_tie_pipe(&String::from("header\n\n")),
        0
    );
    assert_eq!(longest_field_with_last_tie_pipe(&"猫|x\nend|a"), 3);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1129::TRACK_PROOF, 0x52540000 | 1129);
}
