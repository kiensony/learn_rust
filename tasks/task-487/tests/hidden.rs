use task_487::*;

#[test]
fn edge_cases() {
    assert_eq!(
        shortest_field_with_first_tie_pipe(("header\n猫|123", "z|w")),
        3
    );
    assert_eq!(shortest_field_with_first_tie_pipe(("|é|a", "z|w")), 0);
    assert_eq!(shortest_field_with_first_tie_pipe(("", "x")), 1);
    assert_eq!(shortest_field_with_first_tie_pipe(("|", "z|w")), 1);
    assert_eq!(shortest_field_with_first_tie_pipe(("cat", "x")), 3);
    assert_eq!(
        shortest_field_with_first_tie_pipe(("||one|猫|ab", "z|w")),
        0
    );
    assert_eq!(shortest_field_with_first_tie_pipe(("aa|bb", "x")), 2);
    assert_eq!(shortest_field_with_first_tie_pipe(("ok|🐈", "z|w")), 2);
    assert_eq!(
        shortest_field_with_first_tie_pipe(("header\nleft|right\r\n", "x")),
        4
    );
    assert_eq!(shortest_field_with_first_tie_pipe(("header\n\n", "z|w")), 0);
    assert_eq!(shortest_field_with_first_tie_pipe(("猫|x\nend|a", "x")), 1);
}

#[test]
fn submission_marker() {
    assert_eq!(task_487::TRACK_PROOF, 0x52540000 | 487);
}
