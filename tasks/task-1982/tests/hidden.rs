use task_1982::*;

#[test]
fn edge_cases() {
    assert_eq!(atomic_record_high_total(&[9, 2, 2, 8, 0, 5]), 45);
    assert_eq!(atomic_record_high_total(&[0, 0, 0]), 0);
    assert_eq!(atomic_record_high_total(&[255, 1, 255, 2]), 765);
    assert_eq!(
        atomic_record_high_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        145
    );
    assert_eq!(atomic_record_high_total(&[17, 17, 1, 0, 2]), 68);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1982::TRACK_PROOF, 0x52540000 | 1982);
}
