use task_732::*;

#[test]
fn edge_cases() {
    assert_eq!(
        longest_field_with_last_tie_pipe(Some("header\n猫|123")),
        Some(123)
    );
    assert_eq!(longest_field_with_last_tie_pipe(Some("|é|a")), None);
    assert_eq!(longest_field_with_last_tie_pipe(Some("")), None);
    assert_eq!(longest_field_with_last_tie_pipe(Some("|")), None);
    assert_eq!(longest_field_with_last_tie_pipe(Some("cat")), None);
    assert_eq!(longest_field_with_last_tie_pipe(Some("||one|猫|ab")), None);
    assert_eq!(longest_field_with_last_tie_pipe(Some("aa|bb")), None);
    assert_eq!(longest_field_with_last_tie_pipe(Some("ok|🐈")), None);
    assert_eq!(
        longest_field_with_last_tie_pipe(Some("header\nleft|right\r\n")),
        None
    );
    assert_eq!(longest_field_with_last_tie_pipe(Some("header\n\n")), None);
    assert_eq!(longest_field_with_last_tie_pipe(Some("猫|x\nend|a")), None);
    assert_eq!(longest_field_with_last_tie_pipe(None), None);
    assert_eq!(longest_field_with_last_tie_pipe(Some("3|72")), Some(72));
    assert_eq!(longest_field_with_last_tie_pipe(Some("|9")), Some(9));
    assert_eq!(longest_field_with_last_tie_pipe(Some("3300")), Some(3300));
    assert_eq!(longest_field_with_last_tie_pipe(Some("65536")), None);
}

#[test]
fn submission_marker() {
    assert_eq!(task_732::TRACK_PROOF, 0x52540000 | 732);
}
