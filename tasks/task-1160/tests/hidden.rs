use task_1160::*;

#[test]
fn edge_cases() {
    assert_eq!(shortest_field_with_first_tie_colon("", "backup"), "backup");
    assert_eq!(shortest_field_with_first_tie_colon(":", "backup"), "backup");
    assert_eq!(shortest_field_with_first_tie_colon("cat", "backup"), "cat");
    assert_eq!(
        shortest_field_with_first_tie_colon("::one:猫:ab", "backup"),
        "backup"
    );
    assert_eq!(shortest_field_with_first_tie_colon("aa:bb", "backup"), "aa");
    assert_eq!(shortest_field_with_first_tie_colon("ok:🐈", "backup"), "ok");
}

#[test]
fn submission_marker() {
    assert_eq!(task_1160::TRACK_PROOF, 0x52540000 | 1160);
}
