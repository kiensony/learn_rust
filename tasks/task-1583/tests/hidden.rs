use task_1583::*;

#[test]
fn edge_cases() {
    assert_eq!(
        feature_lanes_bounds(&[9, 2, 2, 8, 0, 5]),
        if cfg!(feature = "wide") {
            Some((0u32, 144u32))
        } else {
            Some((0u32, 9u32))
        }
    );
    assert_eq!(
        feature_lanes_bounds(&[0, 0, 0]),
        if cfg!(feature = "wide") {
            Some((0u32, 0u32))
        } else {
            Some((0u32, 0u32))
        }
    );
    assert_eq!(
        feature_lanes_bounds(&[255, 1, 255, 2]),
        if cfg!(feature = "wide") {
            Some((16u32, 4080u32))
        } else {
            Some((1u32, 255u32))
        }
    );
    assert_eq!(
        feature_lanes_bounds(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        if cfg!(feature = "wide") {
            Some((0u32, 496u32))
        } else {
            Some((0u32, 31u32))
        }
    );
    assert_eq!(
        feature_lanes_bounds(&[17, 17, 1, 0, 2]),
        if cfg!(feature = "wide") {
            Some((0u32, 272u32))
        } else {
            Some((0u32, 17u32))
        }
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1583::TRACK_PROOF, 0x52540000 | 1583);
}
