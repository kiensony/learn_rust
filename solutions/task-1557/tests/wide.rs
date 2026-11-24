#![cfg(feature = "wide")]

#[test]
fn feature_units_are_used_by_the_public_report() {
    assert_eq!(
        task_1557::feature_scaling_distinct(&[5, 2, 2, 8, 9, 1, 14, 1, 1]),
        vec![2u32, 4u32, 10u32, 16u32, 18u32, 28u32]
    );
}
