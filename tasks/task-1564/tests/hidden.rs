use task_1564::*;

#[test]
fn edge_cases() {
    assert_eq!(
        feature_padding_peaks(&[9, 2, 2, 8, 0, 5]),
        if cfg!(feature = "wide") {
            vec![36u32]
        } else {
            vec![9u32]
        }
    );
    assert_eq!(
        feature_padding_peaks(&[0, 0, 0]),
        if cfg!(feature = "wide") {
            vec![0u32]
        } else {
            vec![0u32]
        }
    );
    assert_eq!(
        feature_padding_peaks(&[255, 1, 255, 2]),
        if cfg!(feature = "wide") {
            vec![1020u32]
        } else {
            vec![255u32]
        }
    );
    assert_eq!(
        feature_padding_peaks(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        if cfg!(feature = "wide") {
            vec![12u32, 24u32, 36u32, 48u32, 84u32, 104u32, 124u32]
        } else {
            vec![3u32, 6u32, 9u32, 12u32, 21u32, 26u32, 31u32]
        }
    );
    assert_eq!(
        feature_padding_peaks(&[17, 17, 1, 0, 2]),
        if cfg!(feature = "wide") {
            vec![68u32]
        } else {
            vec![17u32]
        }
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1564::TRACK_PROOF, 0x52540000 | 1564);
}
