use task_1968::*;

#[test]
fn edge_cases() {
    assert_eq!(
        atomic_bit_union_steps(&[9, 2, 2, 8, 0, 5]),
        vec![9u32, 2u32, 0u32, 0u32, 0u32]
    );
    assert_eq!(atomic_bit_union_steps(&[0, 0, 0]), vec![0u32, 0u32]);
    assert_eq!(
        atomic_bit_union_steps(&[255, 1, 255, 2]),
        vec![255u32, 0u32, 0u32]
    );
    assert_eq!(
        atomic_bit_union_steps(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![3u32, 4u32, 8u32, 0u32, 16u32, 0u32, 0u32, 0u32, 0u32]
    );
    assert_eq!(
        atomic_bit_union_steps(&[17, 17, 1, 0, 2]),
        vec![17u32, 0u32, 0u32, 0u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1968::TRACK_PROOF, 0x52540000 | 1968);
}
