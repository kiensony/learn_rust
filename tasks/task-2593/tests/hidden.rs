use task_2593::*;

#[test]
fn edge_cases() {
    assert_eq!(
        interior_stride_proof_bounds(&[9, 2, 2, 8, 0, 5]),
        Some((3u32, 6u32))
    );
    assert_eq!(interior_stride_proof_bounds(&[0, 0, 0]), Some((1u32, 1u32)));
    assert_eq!(
        interior_stride_proof_bounds(&[255, 1, 255, 2]),
        Some((0u32, 0u32))
    );
    assert_eq!(
        interior_stride_proof_bounds(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        Some((9u32, 16u32))
    );
    assert_eq!(
        interior_stride_proof_bounds(&[17, 17, 1, 0, 2]),
        Some((2u32, 2u32))
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2593::TRACK_PROOF, 0x52540000 | 2593);
}
