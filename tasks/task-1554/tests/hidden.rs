use task_1554::*;

#[test]
fn edge_cases() {
    assert_eq!(
        feature_scaling_peaks(&[9, 2, 2, 8, 0, 5]),
        if cfg!(feature = "wide") {
            vec![18u32]
        } else {
            vec![9u32]
        }
    );
    assert_eq!(
        feature_scaling_peaks(&[0, 0, 0]),
        if cfg!(feature = "wide") {
            vec![0u32]
        } else {
            vec![0u32]
        }
    );
    assert_eq!(
        feature_scaling_peaks(&[255, 1, 255, 2]),
        if cfg!(feature = "wide") {
            vec![510u32]
        } else {
            vec![255u32]
        }
    );
    assert_eq!(
        feature_scaling_peaks(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        if cfg!(feature = "wide") {
            vec![6u32, 12u32, 18u32, 24u32, 42u32, 52u32, 62u32]
        } else {
            vec![3u32, 6u32, 9u32, 12u32, 21u32, 26u32, 31u32]
        }
    );
    assert_eq!(
        feature_scaling_peaks(&[17, 17, 1, 0, 2]),
        if cfg!(feature = "wide") {
            vec![34u32]
        } else {
            vec![17u32]
        }
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1554::TRACK_PROOF, 0x52540000 | 1554);
}
