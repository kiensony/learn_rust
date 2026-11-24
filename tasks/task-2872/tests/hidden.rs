use task_2872::*;

#[test]
fn edge_cases() {
    assert_eq!(flag_register_total(&[9, 2, 2, 8, 0, 5]), 0);
    assert_eq!(flag_register_total(&[0, 0, 0]), 0);
    assert_eq!(flag_register_total(&[255, 1, 255, 2]), 3);
    assert_eq!(
        flag_register_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        23
    );
    assert_eq!(flag_register_total(&[17, 17, 1, 0, 2]), 17);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2872::TRACK_PROOF, 0x52540000 | 2872);
}
