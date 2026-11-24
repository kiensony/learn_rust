use task_1595::*;

#[test]
fn edge_cases() {
    assert_eq!(
        feature_pages_changes(&[9, 2, 2, 8, 0, 5]),
        if cfg!(feature = "wide") {
            vec![288u32, 64u32, 256u32, 0u32, 160u32]
        } else {
            vec![9u32, 2u32, 8u32, 0u32, 5u32]
        }
    );
    assert_eq!(
        feature_pages_changes(&[0, 0, 0]),
        if cfg!(feature = "wide") {
            vec![0u32]
        } else {
            vec![0u32]
        }
    );
    assert_eq!(
        feature_pages_changes(&[255, 1, 255, 2]),
        if cfg!(feature = "wide") {
            vec![8160u32, 32u32, 8160u32, 64u32]
        } else {
            vec![255u32, 1u32, 255u32, 2u32]
        }
    );
    assert_eq!(
        feature_pages_changes(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        if cfg!(feature = "wide") {
            vec![
                96u32, 192u32, 288u32, 384u32, 672u32, 480u32, 0u32, 832u32, 256u32, 992u32,
            ]
        } else {
            vec![
                3u32, 6u32, 9u32, 12u32, 21u32, 15u32, 0u32, 26u32, 8u32, 31u32,
            ]
        }
    );
    assert_eq!(
        feature_pages_changes(&[17, 17, 1, 0, 2]),
        if cfg!(feature = "wide") {
            vec![544u32, 32u32, 0u32, 64u32]
        } else {
            vec![17u32, 1u32, 0u32, 2u32]
        }
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1595::TRACK_PROOF, 0x52540000 | 1595);
}
