use task_2408::*;

#[test]
fn edge_cases() {
    assert_eq!(
        header_alignment_steps(&[9, 2, 2, 8, 0, 5]),
        vec![8u32, 0u32, 6u32, 8u32, 6u32]
    );
    assert_eq!(header_alignment_steps(&[0, 0, 0]), vec![0u32, 0u32]);
    assert_eq!(
        header_alignment_steps(&[255, 1, 255, 2]),
        vec![254u32, 254u32, 254u32]
    );
    assert_eq!(
        header_alignment_steps(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![2u32, 4u32, 2u32, 10u32, 6u32, 16u32, 26u32, 18u32, 24u32]
    );
    assert_eq!(
        header_alignment_steps(&[17, 17, 1, 0, 2]),
        vec![0u32, 16u32, 2u32, 2u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2408::TRACK_PROOF, 0x52540000 | 2408);
}
