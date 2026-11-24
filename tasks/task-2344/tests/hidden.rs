use task_2344::*;

#[test]
fn edge_cases() {
    assert_eq!(checksum(&[1, 2]), 10);
    assert_eq!(checksum(&[255, 0, 3]), 16323);
    assert_eq!(checksum(&[0, 0, 0]), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2344::TRACK_PROOF, 0x52540000 | 2344);
}
