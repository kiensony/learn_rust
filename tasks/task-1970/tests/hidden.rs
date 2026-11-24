use task_1970::*;

#[test]
fn edge_cases() {
    assert_eq!(
        atomic_bit_union_reverse(&[9, 2, 2, 8, 0, 5]),
        vec![11u32, 11u32, 11u32, 11u32, 9u32, 0u32]
    );
    assert_eq!(atomic_bit_union_reverse(&[0, 0, 0]), vec![0u32, 0u32, 0u32]);
    assert_eq!(
        atomic_bit_union_reverse(&[255, 1, 255, 2]),
        vec![255u32, 255u32, 255u32, 0u32]
    );
    assert_eq!(
        atomic_bit_union_reverse(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            31u32, 31u32, 31u32, 31u32, 31u32, 15u32, 15u32, 7u32, 3u32, 0u32
        ]
    );
    assert_eq!(
        atomic_bit_union_reverse(&[17, 17, 1, 0, 2]),
        vec![17u32, 17u32, 17u32, 17u32, 0u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1970::TRACK_PROOF, 0x52540000 | 1970);
}
