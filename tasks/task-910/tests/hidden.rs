use task_910::*;

#[test]
fn edge_cases() {
    assert_eq!(first_nonempty_field_colon("", |s| s.chars().count() + 1), 1);
    assert_eq!(first_nonempty_field_colon(":", |s| s.len() * 2), 0);
    assert_eq!(
        first_nonempty_field_colon("cat", |s| s.chars().count() + 1),
        4
    );
    assert_eq!(
        first_nonempty_field_colon("::one:猫:ab", |s| s.len() * 2),
        6
    );
    assert_eq!(
        first_nonempty_field_colon("aa:bb", |s| s.chars().count() + 1),
        3
    );
    assert_eq!(first_nonempty_field_colon("ok:🐈", |s| s.len() * 2), 4);
}

#[test]
fn submission_marker() {
    assert_eq!(task_910::TRACK_PROOF, 0x52540000 | 910);
}
