use task_1408::*;

#[test]
fn edge_cases() {
    assert_eq!(
        millisecond_remainder_steps(&[9, 2, 2, 8, 0, 5]),
        vec![119u32, 0u32, 102u32, 136u32, 85u32]
    );
    assert_eq!(millisecond_remainder_steps(&[0, 0, 0]), vec![0u32, 0u32]);
    assert_eq!(
        millisecond_remainder_steps(&[255, 1, 255, 2]),
        vec![318u32, 318u32, 301u32]
    );
    assert_eq!(
        millisecond_remainder_steps(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            51u32, 51u32, 51u32, 153u32, 102u32, 255u32, 442u32, 306u32, 391u32
        ]
    );
    assert_eq!(
        millisecond_remainder_steps(&[17, 17, 1, 0, 2]),
        vec![0u32, 272u32, 17u32, 34u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1408::TRACK_PROOF, 0x52540000 | 1408);
}
