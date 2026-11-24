use task_486::*;

#[test]
fn edge_cases() {
    assert_eq!(
        longest_field_with_last_tie_pipe(("header\n猫|123", "z|w")),
        3
    );
    assert_eq!(longest_field_with_last_tie_pipe(("hi|kit", "x")), 3);
    assert_eq!(longest_field_with_last_tie_pipe(("", "x")), 1);
    assert_eq!(longest_field_with_last_tie_pipe(("|", "z|w")), 1);
    assert_eq!(longest_field_with_last_tie_pipe(("cat", "x")), 3);
    assert_eq!(longest_field_with_last_tie_pipe(("||one|猫|ab", "z|w")), 3);
    assert_eq!(longest_field_with_last_tie_pipe(("aa|bb", "x")), 2);
    assert_eq!(longest_field_with_last_tie_pipe(("ok|🐈", "z|w")), 4);
    assert_eq!(
        longest_field_with_last_tie_pipe(("header\nleft|right\r\n", "x")),
        5
    );
    assert_eq!(longest_field_with_last_tie_pipe(("header\n\n", "z|w")), 0);
    assert_eq!(longest_field_with_last_tie_pipe(("猫|x\nend|a", "x")), 3);
}

#[test]
fn submission_marker() {
    assert_eq!(task_486::TRACK_PROOF, 0x52540000 | 486);
}
