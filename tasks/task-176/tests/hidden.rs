use task_176::*;

#[test]
fn edge_cases() {
    assert_eq!(override_code_3(13, 3), false);
    assert_eq!(override_code_3(0, 6), false);
    assert_eq!(override_code_3(1, 9), false);
    assert_eq!(override_code_3(2, 12), false);
    assert_eq!(override_code_3(3, 15), false);
    assert_eq!(override_code_3(65535, 4), false);
    assert_eq!(override_code_3(5, 7), false);
    assert_eq!(override_code_3(6, 10), false);
    assert_eq!(override_code_3(6, 13), false);
    assert_eq!(override_code_3(7, 16), false);
    assert_eq!(override_code_3(65535, 1), false);
    assert_eq!(override_code_3(0, 0), false);
    assert_eq!(override_code_3(1, 0), false);
    assert_eq!(override_code_3(2, 0), false);
    assert_eq!(override_code_3(3, 0), true);
    assert_eq!(override_code_3(5, 0), true);
    assert_eq!(override_code_3(6, 0), false);
    assert_eq!(override_code_3(7, 0), false);
    assert_eq!(override_code_3(20, 0), false);
    assert_eq!(override_code_3(100, 0), false);
}

#[test]
fn submission_marker() {
    assert_eq!(task_176::TRACK_PROOF, 0x52540000 | 176);
}
