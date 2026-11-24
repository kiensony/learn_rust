use task_907::*;

#[test]
fn edge_cases() {
    assert_eq!(
        marks_removed_at_both_ends_colon("", |s| s.chars().count() + 1),
        1
    );
    assert_eq!(marks_removed_at_both_ends_colon(":", |s| s.len() * 2), 0);
    assert_eq!(
        marks_removed_at_both_ends_colon("cat", |s| s.chars().count() + 1),
        4
    );
    assert_eq!(
        marks_removed_at_both_ends_colon("::one:猫:ab", |s| s.len() * 2),
        20
    );
    assert_eq!(
        marks_removed_at_both_ends_colon("aa:bb", |s| s.chars().count() + 1),
        6
    );
    assert_eq!(
        marks_removed_at_both_ends_colon("ok:🐈", |s| s.len() * 2),
        14
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_907::TRACK_PROOF, 0x52540000 | 907);
}
