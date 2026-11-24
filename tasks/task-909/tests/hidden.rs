use task_909::*;

#[test]
fn edge_cases() {
    assert_eq!(
        prefix_before_final_mark_colon("", |s| s.chars().count() + 1),
        1
    );
    assert_eq!(prefix_before_final_mark_colon(":", |s| s.len() * 2), 0);
    assert_eq!(
        prefix_before_final_mark_colon("cat", |s| s.chars().count() + 1),
        1
    );
    assert_eq!(
        prefix_before_final_mark_colon("::one:猫:ab", |s| s.len() * 2),
        18
    );
    assert_eq!(
        prefix_before_final_mark_colon("aa:bb", |s| s.chars().count() + 1),
        3
    );
    assert_eq!(prefix_before_final_mark_colon("ok:🐈", |s| s.len() * 2), 4);
}

#[test]
fn submission_marker() {
    assert_eq!(task_909::TRACK_PROOF, 0x52540000 | 909);
}
