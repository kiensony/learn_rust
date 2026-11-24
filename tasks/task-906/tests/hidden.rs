use task_906::*;

#[test]
fn edge_cases() {
    assert_eq!(
        closing_marks_removed_colon("", |s| s.chars().count() + 1),
        1
    );
    assert_eq!(closing_marks_removed_colon(":", |s| s.len() * 2), 0);
    assert_eq!(
        closing_marks_removed_colon("cat", |s| s.chars().count() + 1),
        4
    );
    assert_eq!(
        closing_marks_removed_colon("::one:猫:ab", |s| s.len() * 2),
        24
    );
    assert_eq!(
        closing_marks_removed_colon("aa:bb", |s| s.chars().count() + 1),
        6
    );
    assert_eq!(closing_marks_removed_colon("ok:🐈", |s| s.len() * 2), 14);
}

#[test]
fn submission_marker() {
    assert_eq!(task_906::TRACK_PROOF, 0x52540000 | 906);
}
