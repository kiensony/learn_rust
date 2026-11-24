use task_1580::*;

#[test]
fn edge_cases() {
    assert_eq!(
        feature_packing_reverse(&[9, 2, 2, 8, 0, 5]),
        if cfg!(feature = "wide") {
            vec![40u32, 0u32, 64u32, 16u32, 16u32, 72u32]
        } else {
            vec![5u32, 0u32, 8u32, 2u32, 2u32, 9u32]
        }
    );
    assert_eq!(
        feature_packing_reverse(&[0, 0, 0]),
        if cfg!(feature = "wide") {
            vec![0u32, 0u32, 0u32]
        } else {
            vec![0u32, 0u32, 0u32]
        }
    );
    assert_eq!(
        feature_packing_reverse(&[255, 1, 255, 2]),
        if cfg!(feature = "wide") {
            vec![16u32, 2040u32, 8u32, 2040u32]
        } else {
            vec![2u32, 255u32, 1u32, 255u32]
        }
    );
    assert_eq!(
        feature_packing_reverse(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        if cfg!(feature = "wide") {
            vec![
                248u32, 64u32, 208u32, 0u32, 120u32, 168u32, 96u32, 72u32, 48u32, 24u32,
            ]
        } else {
            vec![
                31u32, 8u32, 26u32, 0u32, 15u32, 21u32, 12u32, 9u32, 6u32, 3u32,
            ]
        }
    );
    assert_eq!(
        feature_packing_reverse(&[17, 17, 1, 0, 2]),
        if cfg!(feature = "wide") {
            vec![16u32, 0u32, 8u32, 136u32, 136u32]
        } else {
            vec![2u32, 0u32, 1u32, 17u32, 17u32]
        }
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1580::TRACK_PROOF, 0x52540000 | 1580);
}
