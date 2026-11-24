use task_1180::*;

#[test]
fn edge_cases() {
    assert_eq!(shortest_field_with_first_tie_pipe("hi|kit", "backup"), "hi");
    assert_eq!(
        shortest_field_with_first_tie_pipe("|é|a", "backup"),
        "backup"
    );
    assert_eq!(shortest_field_with_first_tie_pipe("", "backup"), "backup");
    assert_eq!(shortest_field_with_first_tie_pipe("|", "backup"), "backup");
    assert_eq!(shortest_field_with_first_tie_pipe("cat", "backup"), "cat");
    assert_eq!(
        shortest_field_with_first_tie_pipe("||one|猫|ab", "backup"),
        "backup"
    );
    assert_eq!(shortest_field_with_first_tie_pipe("aa|bb", "backup"), "aa");
    assert_eq!(shortest_field_with_first_tie_pipe("ok|🐈", "backup"), "ok");
    assert_eq!(
        shortest_field_with_first_tie_pipe("header\nleft|right\r\n", "backup"),
        "left"
    );
    assert_eq!(
        shortest_field_with_first_tie_pipe("header\n\n", "backup"),
        "backup"
    );
    assert_eq!(
        shortest_field_with_first_tie_pipe("猫|x\nend|a", "backup"),
        "a"
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1180::TRACK_PROOF, 0x52540000 | 1180);
}
