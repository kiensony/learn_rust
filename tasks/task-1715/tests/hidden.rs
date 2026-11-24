use task_1715::*;

#[test]
fn edge_cases() {
    assert_eq!(
        saturating_macro_changes(&[9, 2, 2, 8, 0, 5]),
        vec![13u32, 6u32, 12u32, 4u32, 9u32]
    );
    assert_eq!(saturating_macro_changes(&[0, 0, 0]), vec![4u32]);
    assert_eq!(
        saturating_macro_changes(&[255, 1, 255, 2]),
        vec![255u32, 5u32, 255u32, 6u32]
    );
    assert_eq!(
        saturating_macro_changes(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            7u32, 10u32, 13u32, 16u32, 25u32, 19u32, 4u32, 30u32, 12u32, 35u32
        ]
    );
    assert_eq!(
        saturating_macro_changes(&[17, 17, 1, 0, 2]),
        vec![21u32, 5u32, 4u32, 6u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1715::TRACK_PROOF, 0x52540000 | 1715);
}
