use task_536::*;

#[test]
fn edge_cases() {
    assert_eq!(longest_field_with_last_tie_pipe("header\n猫|123"), 3);
    assert_eq!(longest_field_with_last_tie_pipe("hi|kit"), 3);
    assert_eq!(longest_field_with_last_tie_pipe(""), 0);
    assert_eq!(longest_field_with_last_tie_pipe("|"), 0);
    assert_eq!(longest_field_with_last_tie_pipe("cat"), 3);
    assert_eq!(longest_field_with_last_tie_pipe("||one|猫|ab"), 1);
    assert_eq!(longest_field_with_last_tie_pipe("aa|bb"), 2);
    assert_eq!(longest_field_with_last_tie_pipe("ok|🐈"), 1);
    assert_eq!(
        longest_field_with_last_tie_pipe("header\nleft|right\r\n"),
        5
    );
    assert_eq!(longest_field_with_last_tie_pipe("header\n\n"), 0);
    assert_eq!(longest_field_with_last_tie_pipe("猫|x\nend|a"), 3);
}

#[test]
fn submission_marker() {
    assert_eq!(task_536::TRACK_PROOF, 0x52540000 | 536);
}
