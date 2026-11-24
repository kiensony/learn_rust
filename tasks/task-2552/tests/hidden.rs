use task_2552::*;

#[test]
fn edge_cases() {
    assert_eq!(split_proof_total(&[9, 2, 2, 8, 0, 5]), 14);
    assert_eq!(split_proof_total(&[0, 0, 0]), 2);
    assert_eq!(split_proof_total(&[255, 1, 255, 2]), 0);
    assert_eq!(split_proof_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]), 46);
    assert_eq!(split_proof_total(&[17, 17, 1, 0, 2]), 23);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2552::TRACK_PROOF, 0x52540000 | 2552);
}
