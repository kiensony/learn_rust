use task_1717::*;

#[test]
fn edge_cases() {
    assert_eq!(
        saturating_macro_distinct(&[9, 2, 2, 8, 0, 5]),
        vec![4u32, 6u32, 9u32, 12u32, 13u32]
    );
    assert_eq!(saturating_macro_distinct(&[0, 0, 0]), vec![4u32]);
    assert_eq!(
        saturating_macro_distinct(&[255, 1, 255, 2]),
        vec![5u32, 6u32, 255u32]
    );
    assert_eq!(
        saturating_macro_distinct(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            4u32, 7u32, 10u32, 12u32, 13u32, 16u32, 19u32, 25u32, 30u32, 35u32
        ]
    );
    assert_eq!(
        saturating_macro_distinct(&[17, 17, 1, 0, 2]),
        vec![4u32, 5u32, 6u32, 21u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1717::TRACK_PROOF, 0x52540000 | 1717);
}
