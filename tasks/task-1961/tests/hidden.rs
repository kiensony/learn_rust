use task_1961::*;

#[test]
fn edge_cases() {
    assert_eq!(
        atomic_bit_union_trace(&[9, 2, 2, 8, 0, 5]),
        vec![0u32, 9u32, 11u32, 11u32, 11u32, 11u32]
    );
    assert_eq!(atomic_bit_union_trace(&[0, 0, 0]), vec![0u32, 0u32, 0u32]);
    assert_eq!(
        atomic_bit_union_trace(&[255, 1, 255, 2]),
        vec![0u32, 255u32, 255u32, 255u32]
    );
    assert_eq!(
        atomic_bit_union_trace(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            0u32, 3u32, 7u32, 15u32, 15u32, 31u32, 31u32, 31u32, 31u32, 31u32
        ]
    );
    assert_eq!(
        atomic_bit_union_trace(&[17, 17, 1, 0, 2]),
        vec![0u32, 17u32, 17u32, 17u32, 17u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1961::TRACK_PROOF, 0x52540000 | 1961);
}
