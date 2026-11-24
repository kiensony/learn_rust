use task_1566::*;

#[test]
fn edge_cases() {
    assert_eq!(
        feature_padding_descending_steps(&[9, 2, 2, 8, 0, 5]),
        if cfg!(feature = "wide") { 2 } else { 2 }
    );
    assert_eq!(
        feature_padding_descending_steps(&[0, 0, 0]),
        if cfg!(feature = "wide") { 0 } else { 0 }
    );
    assert_eq!(
        feature_padding_descending_steps(&[255, 1, 255, 2]),
        if cfg!(feature = "wide") { 2 } else { 2 }
    );
    assert_eq!(
        feature_padding_descending_steps(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        if cfg!(feature = "wide") { 3 } else { 3 }
    );
    assert_eq!(
        feature_padding_descending_steps(&[17, 17, 1, 0, 2]),
        if cfg!(feature = "wide") { 2 } else { 2 }
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1566::TRACK_PROOF, 0x52540000 | 1566);
}
