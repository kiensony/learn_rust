use task_2440::*;

#[test]
fn edge_cases() {
    assert_eq!(
        block_alignment_reverse(&[9, 2, 2, 8, 0, 5]),
        vec![16u32, 0u32, 16u32, 16u32, 16u32, 16u32]
    );
    assert_eq!(block_alignment_reverse(&[0, 0, 0]), vec![0u32, 0u32, 0u32]);
    assert_eq!(
        block_alignment_reverse(&[255, 1, 255, 2]),
        vec![16u32, 256u32, 16u32, 256u32]
    );
    assert_eq!(
        block_alignment_reverse(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            32u32, 16u32, 32u32, 0u32, 16u32, 32u32, 16u32, 16u32, 16u32, 16u32
        ]
    );
    assert_eq!(
        block_alignment_reverse(&[17, 17, 1, 0, 2]),
        vec![16u32, 0u32, 16u32, 32u32, 32u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2440::TRACK_PROOF, 0x52540000 | 2440);
}
