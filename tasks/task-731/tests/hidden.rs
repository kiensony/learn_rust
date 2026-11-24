use task_731::*;

#[test]
fn edge_cases() {
    assert_eq!(first_nonempty_field_pipe(Some("hi|kit")), None);
    assert_eq!(first_nonempty_field_pipe(Some("|é|a")), None);
    assert_eq!(first_nonempty_field_pipe(Some("")), None);
    assert_eq!(first_nonempty_field_pipe(Some("|")), None);
    assert_eq!(first_nonempty_field_pipe(Some("cat")), None);
    assert_eq!(first_nonempty_field_pipe(Some("||one|猫|ab")), None);
    assert_eq!(first_nonempty_field_pipe(Some("aa|bb")), None);
    assert_eq!(first_nonempty_field_pipe(Some("ok|🐈")), None);
    assert_eq!(
        first_nonempty_field_pipe(Some("header\nleft|right\r\n")),
        None
    );
    assert_eq!(first_nonempty_field_pipe(Some("header\n\n")), None);
    assert_eq!(first_nonempty_field_pipe(Some("猫|x\nend|a")), None);
    assert_eq!(first_nonempty_field_pipe(None), None);
    assert_eq!(first_nonempty_field_pipe(Some("3|72")), Some(3));
    assert_eq!(first_nonempty_field_pipe(Some("|9")), Some(9));
    assert_eq!(first_nonempty_field_pipe(Some("3300")), Some(3300));
    assert_eq!(first_nonempty_field_pipe(Some("65536")), None);
}

#[test]
fn submission_marker() {
    assert_eq!(task_731::TRACK_PROOF, 0x52540000 | 731);
}
