use task_586::*;

#[test]
fn edge_cases() {
    assert_eq!(shortest_field_with_first_tie_pipe("hi|kit"), "hi");
    assert_eq!(shortest_field_with_first_tie_pipe("|é|a"), "");
    assert_eq!(shortest_field_with_first_tie_pipe(""), "");
    assert_eq!(shortest_field_with_first_tie_pipe("|"), "");
    assert_eq!(shortest_field_with_first_tie_pipe("cat"), "cat");
    assert_eq!(shortest_field_with_first_tie_pipe("||one|猫|ab"), "");
    assert_eq!(shortest_field_with_first_tie_pipe("aa|bb"), "aa");
    assert_eq!(shortest_field_with_first_tie_pipe("ok|🐈"), "ok");
    assert_eq!(
        shortest_field_with_first_tie_pipe("header\nleft|right\r\n"),
        "left"
    );
    assert_eq!(shortest_field_with_first_tie_pipe("header\n\n"), "");
    assert_eq!(shortest_field_with_first_tie_pipe("猫|x\nend|a"), "a");
}

#[test]
fn submission_marker() {
    assert_eq!(task_586::TRACK_PROOF, 0x52540000 | 586);
}
