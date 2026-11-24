use task_1720::*;

#[test]
fn edge_cases() {
    assert_eq!(
        saturating_macro_reverse(&[9, 2, 2, 8, 0, 5]),
        vec![9u32, 4u32, 12u32, 6u32, 6u32, 13u32]
    );
    assert_eq!(saturating_macro_reverse(&[0, 0, 0]), vec![4u32, 4u32, 4u32]);
    assert_eq!(
        saturating_macro_reverse(&[255, 1, 255, 2]),
        vec![6u32, 255u32, 5u32, 255u32]
    );
    assert_eq!(
        saturating_macro_reverse(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            35u32, 12u32, 30u32, 4u32, 19u32, 25u32, 16u32, 13u32, 10u32, 7u32
        ]
    );
    assert_eq!(
        saturating_macro_reverse(&[17, 17, 1, 0, 2]),
        vec![6u32, 4u32, 5u32, 21u32, 21u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1720::TRACK_PROOF, 0x52540000 | 1720);
}
