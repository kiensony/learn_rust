use task_882::*;

#[test]
fn edge_cases() {
    assert_eq!(shortest_field_with_first_tie_pipe("header\n猫|123"), 5);
    assert_eq!(shortest_field_with_first_tie_pipe("|é|a"), 2);
    assert_eq!(shortest_field_with_first_tie_pipe(""), 2);
    assert_eq!(shortest_field_with_first_tie_pipe("|"), 2);
    assert_eq!(shortest_field_with_first_tie_pipe("cat"), 5);
    assert_eq!(shortest_field_with_first_tie_pipe("||one|猫|ab"), 2);
    assert_eq!(shortest_field_with_first_tie_pipe("aa|bb"), 4);
    assert_eq!(shortest_field_with_first_tie_pipe("ok|🐈"), 4);
    assert_eq!(
        shortest_field_with_first_tie_pipe("header\nleft|right\r\n"),
        6
    );
    assert_eq!(shortest_field_with_first_tie_pipe("header\n\n"), 2);
    assert_eq!(shortest_field_with_first_tie_pipe("猫|x\nend|a"), 3);
}

#[test]
fn submission_marker() {
    assert_eq!(task_882::TRACK_PROOF, 0x52540000 | 882);
}
