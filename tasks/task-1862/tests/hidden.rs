use task_1862::*;

#[test]
fn edge_cases() {
    assert_eq!(weak_read_handles_total(&[9, 2, 2, 8, 0, 5]), 6);
    assert_eq!(weak_read_handles_total(&[0, 0, 0]), 0);
    assert_eq!(weak_read_handles_total(&[255, 1, 255, 2]), 9);
    assert_eq!(
        weak_read_handles_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        15
    );
    assert_eq!(weak_read_handles_total(&[17, 17, 1, 0, 2]), 5);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1862::TRACK_PROOF, 0x52540000 | 1862);
}
