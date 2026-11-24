use task_1567::*;

#[test]
fn edge_cases() {
    assert_eq!(
        feature_padding_distinct(&[9, 2, 2, 8, 0, 5]),
        if cfg!(feature = "wide") {
            vec![0u32, 8u32, 20u32, 32u32, 36u32]
        } else {
            vec![0u32, 2u32, 5u32, 8u32, 9u32]
        }
    );
    assert_eq!(
        feature_padding_distinct(&[0, 0, 0]),
        if cfg!(feature = "wide") {
            vec![0u32]
        } else {
            vec![0u32]
        }
    );
    assert_eq!(
        feature_padding_distinct(&[255, 1, 255, 2]),
        if cfg!(feature = "wide") {
            vec![4u32, 8u32, 1020u32]
        } else {
            vec![1u32, 2u32, 255u32]
        }
    );
    assert_eq!(
        feature_padding_distinct(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        if cfg!(feature = "wide") {
            vec![
                0u32, 12u32, 24u32, 32u32, 36u32, 48u32, 60u32, 84u32, 104u32, 124u32,
            ]
        } else {
            vec![
                0u32, 3u32, 6u32, 8u32, 9u32, 12u32, 15u32, 21u32, 26u32, 31u32,
            ]
        }
    );
    assert_eq!(
        feature_padding_distinct(&[17, 17, 1, 0, 2]),
        if cfg!(feature = "wide") {
            vec![0u32, 4u32, 8u32, 68u32]
        } else {
            vec![0u32, 1u32, 2u32, 17u32]
        }
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1567::TRACK_PROOF, 0x52540000 | 1567);
}
