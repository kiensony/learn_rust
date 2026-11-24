use task_1757::*;

#[test]
fn edge_cases() {
    assert_eq!(
        build_multiplier_distinct(&[9, 2, 2, 8, 0, 5]),
        vec![0u32, 6u32, 15u32, 24u32, 27u32]
    );
    assert_eq!(build_multiplier_distinct(&[0, 0, 0]), vec![0u32]);
    assert_eq!(
        build_multiplier_distinct(&[255, 1, 255, 2]),
        vec![3u32, 6u32, 765u32]
    );
    assert_eq!(
        build_multiplier_distinct(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            0u32, 9u32, 18u32, 24u32, 27u32, 36u32, 45u32, 63u32, 78u32, 93u32
        ]
    );
    assert_eq!(
        build_multiplier_distinct(&[17, 17, 1, 0, 2]),
        vec![0u32, 3u32, 6u32, 51u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1757::TRACK_PROOF, 0x52540000 | 1757);
}
