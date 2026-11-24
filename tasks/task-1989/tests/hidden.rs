use task_1989::*;

#[test]
fn edge_cases() {
    assert_eq!(atomic_record_high_longest_run(&[9, 2, 2, 8, 0, 5]), 5);
    assert_eq!(atomic_record_high_longest_run(&[0, 0, 0]), 3);
    assert_eq!(atomic_record_high_longest_run(&[255, 1, 255, 2]), 3);
    assert_eq!(
        atomic_record_high_longest_run(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        3
    );
    assert_eq!(atomic_record_high_longest_run(&[17, 17, 1, 0, 2]), 4);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1989::TRACK_PROOF, 0x52540000 | 1989);
}
