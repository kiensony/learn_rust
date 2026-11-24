use task_1570::*;

#[test]
fn edge_cases() {
    assert_eq!(
        feature_padding_reverse(&[9, 2, 2, 8, 0, 5]),
        if cfg!(feature = "wide") {
            vec![20u32, 0u32, 32u32, 8u32, 8u32, 36u32]
        } else {
            vec![5u32, 0u32, 8u32, 2u32, 2u32, 9u32]
        }
    );
    assert_eq!(
        feature_padding_reverse(&[0, 0, 0]),
        if cfg!(feature = "wide") {
            vec![0u32, 0u32, 0u32]
        } else {
            vec![0u32, 0u32, 0u32]
        }
    );
    assert_eq!(
        feature_padding_reverse(&[255, 1, 255, 2]),
        if cfg!(feature = "wide") {
            vec![8u32, 1020u32, 4u32, 1020u32]
        } else {
            vec![2u32, 255u32, 1u32, 255u32]
        }
    );
    assert_eq!(
        feature_padding_reverse(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        if cfg!(feature = "wide") {
            vec![
                124u32, 32u32, 104u32, 0u32, 60u32, 84u32, 48u32, 36u32, 24u32, 12u32,
            ]
        } else {
            vec![
                31u32, 8u32, 26u32, 0u32, 15u32, 21u32, 12u32, 9u32, 6u32, 3u32,
            ]
        }
    );
    assert_eq!(
        feature_padding_reverse(&[17, 17, 1, 0, 2]),
        if cfg!(feature = "wide") {
            vec![8u32, 0u32, 4u32, 68u32, 68u32]
        } else {
            vec![2u32, 0u32, 1u32, 17u32, 17u32]
        }
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1570::TRACK_PROOF, 0x52540000 | 1570);
}
