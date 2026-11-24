use task_862::*;

#[test]
fn edge_cases() {
    assert_eq!(shortest_field_with_first_tie_colon(""), 2);
    assert_eq!(shortest_field_with_first_tie_colon(":"), 2);
    assert_eq!(shortest_field_with_first_tie_colon("cat"), 5);
    assert_eq!(shortest_field_with_first_tie_colon("::one:猫:ab"), 2);
    assert_eq!(shortest_field_with_first_tie_colon("aa:bb"), 4);
    assert_eq!(shortest_field_with_first_tie_colon("ok:🐈"), 4);
}

#[test]
fn submission_marker() {
    assert_eq!(task_862::TRACK_PROOF, 0x52540000 | 862);
}
