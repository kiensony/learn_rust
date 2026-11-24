use task_2421::*;

#[test]
fn edge_cases() {
    assert_eq!(
        wide_alignment_trace(&[9, 2, 2, 8, 0, 5]),
        vec![16u32, 8u32, 8u32, 8u32, 0u32, 8u32]
    );
    assert_eq!(wide_alignment_trace(&[0, 0, 0]), vec![0u32, 0u32, 0u32]);
    assert_eq!(
        wide_alignment_trace(&[255, 1, 255, 2]),
        vec![256u32, 8u32, 256u32, 8u32]
    );
    assert_eq!(
        wide_alignment_trace(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            8u32, 8u32, 16u32, 16u32, 24u32, 16u32, 0u32, 32u32, 8u32, 32u32
        ]
    );
    assert_eq!(
        wide_alignment_trace(&[17, 17, 1, 0, 2]),
        vec![24u32, 24u32, 8u32, 0u32, 8u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2421::TRACK_PROOF, 0x52540000 | 2421);
}
