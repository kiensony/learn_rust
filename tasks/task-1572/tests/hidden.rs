use task_1572::*;

#[test]
fn edge_cases() {
    assert_eq!(
        feature_packing_total(&[9, 2, 2, 8, 0, 5]),
        if cfg!(feature = "wide") { 208 } else { 26 }
    );
    assert_eq!(
        feature_packing_total(&[0, 0, 0]),
        if cfg!(feature = "wide") { 0 } else { 0 }
    );
    assert_eq!(
        feature_packing_total(&[255, 1, 255, 2]),
        if cfg!(feature = "wide") { 4104 } else { 513 }
    );
    assert_eq!(
        feature_packing_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        if cfg!(feature = "wide") { 1048 } else { 131 }
    );
    assert_eq!(
        feature_packing_total(&[17, 17, 1, 0, 2]),
        if cfg!(feature = "wide") { 296 } else { 37 }
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1572::TRACK_PROOF, 0x52540000 | 1572);
}
