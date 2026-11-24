#![cfg(feature = "wide")]

#[test]
fn feature_units_are_used_by_the_public_report() {
    assert_eq!(
        task_1567::feature_padding_distinct(&[5, 2, 2, 8, 9, 1, 14, 1, 1]),
        vec![4u32, 8u32, 20u32, 32u32, 36u32, 56u32]
    );
}
