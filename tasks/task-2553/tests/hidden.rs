use task_2553::*;

#[test]
fn edge_cases() {
    assert_eq!(split_proof_bounds(&[9, 2, 2, 8, 0, 5]), Some((1u32, 10u32)));
    assert_eq!(split_proof_bounds(&[0, 0, 0]), Some((1u32, 1u32)));
    assert_eq!(split_proof_bounds(&[255, 1, 255, 2]), Some((0u32, 0u32)));
    assert_eq!(
        split_proof_bounds(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        Some((1u32, 22u32))
    );
    assert_eq!(split_proof_bounds(&[17, 17, 1, 0, 2]), Some((2u32, 18u32)));
}

#[test]
fn submission_marker() {
    assert_eq!(task_2553::TRACK_PROOF, 0x52540000 | 2553);
}
