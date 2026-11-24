use task_2390::*;

#[test]
fn edge_cases() {
    assert_eq!(
        rolling_parity_reverse(&[9, 2, 2, 8, 0, 5]),
        vec![4u32, 1u32, 1u32, 9u32, 11u32, 9u32]
    );
    assert_eq!(rolling_parity_reverse(&[0, 0, 0]), vec![0u32, 0u32, 0u32]);
    assert_eq!(
        rolling_parity_reverse(&[255, 1, 255, 2]),
        vec![3u32, 1u32, 254u32, 255u32]
    );
    assert_eq!(
        rolling_parity_reverse(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            23u32, 8u32, 0u32, 26u32, 26u32, 21u32, 0u32, 12u32, 5u32, 3u32
        ]
    );
    assert_eq!(
        rolling_parity_reverse(&[17, 17, 1, 0, 2]),
        vec![3u32, 1u32, 1u32, 0u32, 17u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2390::TRACK_PROOF, 0x52540000 | 2390);
}
