use task_1760::*;

#[test]
fn edge_cases() {
    assert_eq!(
        build_multiplier_reverse(&[9, 2, 2, 8, 0, 5]),
        vec![15u32, 0u32, 24u32, 6u32, 6u32, 27u32]
    );
    assert_eq!(build_multiplier_reverse(&[0, 0, 0]), vec![0u32, 0u32, 0u32]);
    assert_eq!(
        build_multiplier_reverse(&[255, 1, 255, 2]),
        vec![6u32, 765u32, 3u32, 765u32]
    );
    assert_eq!(
        build_multiplier_reverse(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            93u32, 24u32, 78u32, 0u32, 45u32, 63u32, 36u32, 27u32, 18u32, 9u32
        ]
    );
    assert_eq!(
        build_multiplier_reverse(&[17, 17, 1, 0, 2]),
        vec![6u32, 0u32, 3u32, 51u32, 51u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1760::TRACK_PROOF, 0x52540000 | 1760);
}
