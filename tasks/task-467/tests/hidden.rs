use task_467::*;

#[test]
fn edge_cases() {
    assert_eq!(shortest_field_with_first_tie_colon(("", "x")), 1);
    assert_eq!(shortest_field_with_first_tie_colon((":", "z:w")), 1);
    assert_eq!(shortest_field_with_first_tie_colon(("cat", "x")), 3);
    assert_eq!(
        shortest_field_with_first_tie_colon(("::one:猫:ab", "z:w")),
        0
    );
    assert_eq!(shortest_field_with_first_tie_colon(("aa:bb", "x")), 2);
    assert_eq!(shortest_field_with_first_tie_colon(("ok:🐈", "z:w")), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_467::TRACK_PROOF, 0x52540000 | 467);
}
