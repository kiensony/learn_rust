use task_1558::*;

#[test]
fn edge_cases() {
    assert_eq!(
        feature_scaling_steps(&[9, 2, 2, 8, 0, 5]),
        if cfg!(feature = "wide") {
            vec![14u32, 0u32, 12u32, 16u32, 10u32]
        } else {
            vec![7u32, 0u32, 6u32, 8u32, 5u32]
        }
    );
    assert_eq!(
        feature_scaling_steps(&[0, 0, 0]),
        if cfg!(feature = "wide") {
            vec![0u32, 0u32]
        } else {
            vec![0u32, 0u32]
        }
    );
    assert_eq!(
        feature_scaling_steps(&[255, 1, 255, 2]),
        if cfg!(feature = "wide") {
            vec![508u32, 508u32, 506u32]
        } else {
            vec![254u32, 254u32, 253u32]
        }
    );
    assert_eq!(
        feature_scaling_steps(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        if cfg!(feature = "wide") {
            vec![6u32, 6u32, 6u32, 18u32, 12u32, 30u32, 52u32, 36u32, 46u32]
        } else {
            vec![3u32, 3u32, 3u32, 9u32, 6u32, 15u32, 26u32, 18u32, 23u32]
        }
    );
    assert_eq!(
        feature_scaling_steps(&[17, 17, 1, 0, 2]),
        if cfg!(feature = "wide") {
            vec![0u32, 32u32, 2u32, 4u32]
        } else {
            vec![0u32, 16u32, 1u32, 2u32]
        }
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1558::TRACK_PROOF, 0x52540000 | 1558);
}
