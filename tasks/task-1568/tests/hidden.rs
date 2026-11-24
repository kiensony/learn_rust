use task_1568::*;

#[test]
fn edge_cases() {
    assert_eq!(
        feature_padding_steps(&[9, 2, 2, 8, 0, 5]),
        if cfg!(feature = "wide") {
            vec![28u32, 0u32, 24u32, 32u32, 20u32]
        } else {
            vec![7u32, 0u32, 6u32, 8u32, 5u32]
        }
    );
    assert_eq!(
        feature_padding_steps(&[0, 0, 0]),
        if cfg!(feature = "wide") {
            vec![0u32, 0u32]
        } else {
            vec![0u32, 0u32]
        }
    );
    assert_eq!(
        feature_padding_steps(&[255, 1, 255, 2]),
        if cfg!(feature = "wide") {
            vec![1016u32, 1016u32, 1012u32]
        } else {
            vec![254u32, 254u32, 253u32]
        }
    );
    assert_eq!(
        feature_padding_steps(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        if cfg!(feature = "wide") {
            vec![
                12u32, 12u32, 12u32, 36u32, 24u32, 60u32, 104u32, 72u32, 92u32,
            ]
        } else {
            vec![3u32, 3u32, 3u32, 9u32, 6u32, 15u32, 26u32, 18u32, 23u32]
        }
    );
    assert_eq!(
        feature_padding_steps(&[17, 17, 1, 0, 2]),
        if cfg!(feature = "wide") {
            vec![0u32, 64u32, 4u32, 8u32]
        } else {
            vec![0u32, 16u32, 1u32, 2u32]
        }
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1568::TRACK_PROOF, 0x52540000 | 1568);
}
