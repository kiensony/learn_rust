use task_2423::*;

#[test]
fn edge_cases() {
    assert_eq!(
        wide_alignment_bounds(&[9, 2, 2, 8, 0, 5]),
        Some((0u32, 16u32))
    );
    assert_eq!(wide_alignment_bounds(&[0, 0, 0]), Some((0u32, 0u32)));
    assert_eq!(
        wide_alignment_bounds(&[255, 1, 255, 2]),
        Some((8u32, 256u32))
    );
    assert_eq!(
        wide_alignment_bounds(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        Some((0u32, 32u32))
    );
    assert_eq!(
        wide_alignment_bounds(&[17, 17, 1, 0, 2]),
        Some((0u32, 24u32))
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2423::TRACK_PROOF, 0x52540000 | 2423);
}
