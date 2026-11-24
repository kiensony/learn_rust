use task_1992::*;

#[test]
fn edge_cases() {
    assert_eq!(atomic_exchange_total(&[9, 2, 2, 8, 0, 5]), 21);
    assert_eq!(atomic_exchange_total(&[0, 0, 0]), 0);
    assert_eq!(atomic_exchange_total(&[255, 1, 255, 2]), 511);
    assert_eq!(
        atomic_exchange_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        100
    );
    assert_eq!(atomic_exchange_total(&[17, 17, 1, 0, 2]), 35);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1992::TRACK_PROOF, 0x52540000 | 1992);
}
