use task_912::*;

#[test]
fn edge_cases() {
    assert_eq!(
        shortest_field_with_first_tie_colon("", |s| s.chars().count() + 1),
        1
    );
    assert_eq!(shortest_field_with_first_tie_colon(":", |s| s.len() * 2), 0);
    assert_eq!(
        shortest_field_with_first_tie_colon("cat", |s| s.chars().count() + 1),
        4
    );
    assert_eq!(
        shortest_field_with_first_tie_colon("::one:猫:ab", |s| s.len() * 2),
        0
    );
    assert_eq!(
        shortest_field_with_first_tie_colon("aa:bb", |s| s.chars().count() + 1),
        3
    );
    assert_eq!(
        shortest_field_with_first_tie_colon("ok:🐈", |s| s.len() * 2),
        4
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_912::TRACK_PROOF, 0x52540000 | 912);
}
