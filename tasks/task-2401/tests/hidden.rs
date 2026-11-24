use task_2401::*;

#[test]
fn edge_cases() {
    assert_eq!(
        header_alignment_trace(&[9, 2, 2, 8, 0, 5]),
        vec![10u32, 2u32, 2u32, 8u32, 0u32, 6u32]
    );
    assert_eq!(header_alignment_trace(&[0, 0, 0]), vec![0u32, 0u32, 0u32]);
    assert_eq!(
        header_alignment_trace(&[255, 1, 255, 2]),
        vec![256u32, 2u32, 256u32, 2u32]
    );
    assert_eq!(
        header_alignment_trace(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            4u32, 6u32, 10u32, 12u32, 22u32, 16u32, 0u32, 26u32, 8u32, 32u32
        ]
    );
    assert_eq!(
        header_alignment_trace(&[17, 17, 1, 0, 2]),
        vec![18u32, 18u32, 2u32, 0u32, 2u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2401::TRACK_PROOF, 0x52540000 | 2401);
}
