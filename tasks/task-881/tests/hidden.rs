use task_881::*;

#[test]
fn edge_cases() {
    assert_eq!(longest_field_with_last_tie_pipe("header\n猫|123"), 5);
    assert_eq!(longest_field_with_last_tie_pipe("hi|kit"), 5);
    assert_eq!(longest_field_with_last_tie_pipe(""), 2);
    assert_eq!(longest_field_with_last_tie_pipe("|"), 2);
    assert_eq!(longest_field_with_last_tie_pipe("cat"), 5);
    assert_eq!(longest_field_with_last_tie_pipe("||one|猫|ab"), 5);
    assert_eq!(longest_field_with_last_tie_pipe("aa|bb"), 4);
    assert_eq!(longest_field_with_last_tie_pipe("ok|🐈"), 6);
    assert_eq!(
        longest_field_with_last_tie_pipe("header\nleft|right\r\n"),
        7
    );
    assert_eq!(longest_field_with_last_tie_pipe("header\n\n"), 2);
    assert_eq!(longest_field_with_last_tie_pipe("猫|x\nend|a"), 5);
}

#[test]
fn submission_marker() {
    assert_eq!(task_881::TRACK_PROOF, 0x52540000 | 881);
}
