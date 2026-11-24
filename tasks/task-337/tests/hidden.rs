use task_337::*;

#[test]
fn edge_cases() {
    assert_eq!(
        shortest_field_with_first_tie_pipe(String::from("hi|kit")),
        "hi"
    );
    assert_eq!(shortest_field_with_first_tie_pipe(String::from("|é|a")), "");
    assert_eq!(shortest_field_with_first_tie_pipe(String::from("")), "");
    assert_eq!(shortest_field_with_first_tie_pipe(String::from("|")), "");
    assert_eq!(
        shortest_field_with_first_tie_pipe(String::from("cat")),
        "cat"
    );
    assert_eq!(
        shortest_field_with_first_tie_pipe(String::from("||one|猫|ab")),
        ""
    );
    assert_eq!(
        shortest_field_with_first_tie_pipe(String::from("aa|bb")),
        "aa"
    );
    assert_eq!(
        shortest_field_with_first_tie_pipe(String::from("ok|🐈")),
        "ok"
    );
    assert_eq!(
        shortest_field_with_first_tie_pipe(String::from("header\nleft|right\r\n")),
        "left"
    );
    assert_eq!(
        shortest_field_with_first_tie_pipe(String::from("header\n\n")),
        ""
    );
    assert_eq!(
        shortest_field_with_first_tie_pipe(String::from("猫|x\nend|a")),
        "a"
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_337::TRACK_PROOF, 0x52540000 | 337);
}
