use task_861::*;

#[test]
fn edge_cases() {
    assert_eq!(longest_field_with_last_tie_colon(""), 2);
    assert_eq!(longest_field_with_last_tie_colon(":"), 2);
    assert_eq!(longest_field_with_last_tie_colon("cat"), 5);
    assert_eq!(longest_field_with_last_tie_colon("::one:猫:ab"), 5);
    assert_eq!(longest_field_with_last_tie_colon("aa:bb"), 4);
    assert_eq!(longest_field_with_last_tie_colon("ok:🐈"), 6);
}

#[test]
fn submission_marker() {
    assert_eq!(task_861::TRACK_PROOF, 0x52540000 | 861);
}
