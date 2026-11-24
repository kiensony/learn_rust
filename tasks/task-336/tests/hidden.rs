use task_336::*;

#[test]
fn edge_cases() {
    assert_eq!(
        longest_field_with_last_tie_pipe(String::from("header\n猫|123")),
        "123"
    );
    assert_eq!(longest_field_with_last_tie_pipe(String::from("|é|a")), "é");
    assert_eq!(longest_field_with_last_tie_pipe(String::from("")), "");
    assert_eq!(longest_field_with_last_tie_pipe(String::from("|")), "");
    assert_eq!(longest_field_with_last_tie_pipe(String::from("cat")), "cat");
    assert_eq!(
        longest_field_with_last_tie_pipe(String::from("||one|猫|ab")),
        "猫"
    );
    assert_eq!(
        longest_field_with_last_tie_pipe(String::from("aa|bb")),
        "bb"
    );
    assert_eq!(
        longest_field_with_last_tie_pipe(String::from("ok|🐈")),
        "🐈"
    );
    assert_eq!(
        longest_field_with_last_tie_pipe(String::from("header\nleft|right\r\n")),
        "right"
    );
    assert_eq!(
        longest_field_with_last_tie_pipe(String::from("header\n\n")),
        ""
    );
    assert_eq!(
        longest_field_with_last_tie_pipe(String::from("猫|x\nend|a")),
        "end"
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_336::TRACK_PROOF, 0x52540000 | 336);
}
