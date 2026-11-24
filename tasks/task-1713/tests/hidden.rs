use task_1713::*;

#[test]
fn edge_cases() {
    assert_eq!(
        saturating_macro_bounds(&[9, 2, 2, 8, 0, 5]),
        Some((4u32, 13u32))
    );
    assert_eq!(saturating_macro_bounds(&[0, 0, 0]), Some((4u32, 4u32)));
    assert_eq!(
        saturating_macro_bounds(&[255, 1, 255, 2]),
        Some((5u32, 255u32))
    );
    assert_eq!(
        saturating_macro_bounds(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        Some((4u32, 35u32))
    );
    assert_eq!(
        saturating_macro_bounds(&[17, 17, 1, 0, 2]),
        Some((4u32, 21u32))
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1713::TRACK_PROOF, 0x52540000 | 1713);
}
