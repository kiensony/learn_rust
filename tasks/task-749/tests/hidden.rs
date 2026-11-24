use task_749::*;

#[test]
fn edge_cases() {
    assert_eq!(
        remainder_after_first_mark_hash(Some("[#65535]")),
        Some(65535)
    );
    assert_eq!(remainder_after_first_mark_hash(Some("[#65536]")), None);
    assert_eq!(remainder_after_first_mark_hash(Some("unclosed[cat")), None);
    assert_eq!(remainder_after_first_mark_hash(Some("[]")), None);
    assert_eq!(remainder_after_first_mark_hash(Some("[42]")), None);
    assert_eq!(remainder_after_first_mark_hash(Some("[321]")), None);
    assert_eq!(remainder_after_first_mark_hash(Some("hi#kit")), None);
    assert_eq!(remainder_after_first_mark_hash(Some("#é#a")), None);
    assert_eq!(remainder_after_first_mark_hash(Some("")), None);
    assert_eq!(remainder_after_first_mark_hash(Some("#")), None);
    assert_eq!(remainder_after_first_mark_hash(Some("cat")), None);
    assert_eq!(remainder_after_first_mark_hash(Some("##one#猫#ab")), None);
    assert_eq!(remainder_after_first_mark_hash(Some("aa#bb")), None);
    assert_eq!(remainder_after_first_mark_hash(Some("ok#🐈")), None);
    assert_eq!(remainder_after_first_mark_hash(None), None);
    assert_eq!(remainder_after_first_mark_hash(Some("3#72")), None);
    assert_eq!(remainder_after_first_mark_hash(Some("#9")), None);
    assert_eq!(remainder_after_first_mark_hash(Some("3300")), None);
}

#[test]
fn submission_marker() {
    assert_eq!(task_749::TRACK_PROOF, 0x52540000 | 749);
}
