use task_1562::*;

#[test]
fn edge_cases() {
    assert_eq!(
        feature_padding_total(&[9, 2, 2, 8, 0, 5]),
        if cfg!(feature = "wide") { 104 } else { 26 }
    );
    assert_eq!(
        feature_padding_total(&[0, 0, 0]),
        if cfg!(feature = "wide") { 0 } else { 0 }
    );
    assert_eq!(
        feature_padding_total(&[255, 1, 255, 2]),
        if cfg!(feature = "wide") { 2052 } else { 513 }
    );
    assert_eq!(
        feature_padding_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        if cfg!(feature = "wide") { 524 } else { 131 }
    );
    assert_eq!(
        feature_padding_total(&[17, 17, 1, 0, 2]),
        if cfg!(feature = "wide") { 148 } else { 37 }
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1562::TRACK_PROOF, 0x52540000 | 1562);
}
