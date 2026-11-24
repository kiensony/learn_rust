#![cfg(feature = "wide")]

#[test]
fn feature_units_are_used_by_the_public_report() {
    assert_eq!(
        task_1579::feature_packing_longest_run(&[5, 2, 2, 8, 9, 1, 14, 1, 1]),
        2
    );
}
