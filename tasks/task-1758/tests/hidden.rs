use task_1758::*;

#[test]
fn edge_cases() {
    assert_eq!(
        build_multiplier_steps(&[9, 2, 2, 8, 0, 5]),
        vec![21u32, 0u32, 18u32, 24u32, 15u32]
    );
    assert_eq!(build_multiplier_steps(&[0, 0, 0]), vec![0u32, 0u32]);
    assert_eq!(
        build_multiplier_steps(&[255, 1, 255, 2]),
        vec![762u32, 762u32, 759u32]
    );
    assert_eq!(
        build_multiplier_steps(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![9u32, 9u32, 9u32, 27u32, 18u32, 45u32, 78u32, 54u32, 69u32]
    );
    assert_eq!(
        build_multiplier_steps(&[17, 17, 1, 0, 2]),
        vec![0u32, 48u32, 3u32, 6u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1758::TRACK_PROOF, 0x52540000 | 1758);
}
