use task_384::*;

#[test]
fn edge_cases() {
    assert_eq!(mirror_count(&[]), 0);
    assert_eq!(mirror_count(&[7]), 0);
    assert_eq!(mirror_count(&[1, 2, 3, 1]), 1);
    assert_eq!(mirror_count(&[2, 3, 4, 5]), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_384::TRACK_PROOF, 0x52540000 | 384);
}
