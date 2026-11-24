use task_948::*;

#[test]
fn edge_cases() {
    assert_eq!(
        remainder_after_first_mark_hash("unclosed[cat", |s| s.chars().count() + 1),
        1
    );
    assert_eq!(remainder_after_first_mark_hash("[]", |s| s.len() * 2), 0);
    assert_eq!(
        remainder_after_first_mark_hash("[42]", |s| s.chars().count() + 1),
        1
    );
    assert_eq!(remainder_after_first_mark_hash("[321]", |s| s.len() * 2), 0);
    assert_eq!(
        remainder_after_first_mark_hash("hi#kit", |s| s.chars().count() + 1),
        1
    );
    assert_eq!(remainder_after_first_mark_hash("#é#a", |s| s.len() * 2), 0);
    assert_eq!(
        remainder_after_first_mark_hash("", |s| s.chars().count() + 1),
        1
    );
    assert_eq!(remainder_after_first_mark_hash("#", |s| s.len() * 2), 0);
    assert_eq!(
        remainder_after_first_mark_hash("cat", |s| s.chars().count() + 1),
        1
    );
    assert_eq!(
        remainder_after_first_mark_hash("##one#猫#ab", |s| s.len() * 2),
        0
    );
    assert_eq!(
        remainder_after_first_mark_hash("aa#bb", |s| s.chars().count() + 1),
        1
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_948::TRACK_PROOF, 0x52540000 | 948);
}
