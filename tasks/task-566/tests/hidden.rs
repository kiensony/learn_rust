use task_566::*;

#[test]
fn edge_cases() {
    assert_eq!(shortest_field_with_first_tie_colon(""), "");
    assert_eq!(shortest_field_with_first_tie_colon(":"), "");
    assert_eq!(shortest_field_with_first_tie_colon("cat"), "cat");
    assert_eq!(shortest_field_with_first_tie_colon("::one:猫:ab"), "");
    assert_eq!(shortest_field_with_first_tie_colon("aa:bb"), "aa");
    assert_eq!(shortest_field_with_first_tie_colon("ok:🐈"), "ok");
}

#[test]
fn submission_marker() {
    assert_eq!(task_566::TRACK_PROOF, 0x52540000 | 566);
}
