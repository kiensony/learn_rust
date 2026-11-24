use task_166::*;

#[test]
fn edge_cases() {
    assert_eq!(override_code_2(0), false);
    assert_eq!(override_code_2(1), false);
    assert_eq!(override_code_2(1), false);
    assert_eq!(override_code_2(2), true);
    assert_eq!(override_code_2(3), false);
    assert_eq!(override_code_2(65535), false);
    assert_eq!(override_code_2(4), true);
    assert_eq!(override_code_2(5), false);
    assert_eq!(override_code_2(4), true);
    assert_eq!(override_code_2(5), false);
}

#[test]
fn submission_marker() {
    assert_eq!(task_166::TRACK_PROOF, 0x52540000 | 166);
}
