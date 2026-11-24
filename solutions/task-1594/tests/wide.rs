#![cfg(feature = "wide")]

#[test]
fn feature_units_are_used_by_the_public_report() {
    assert_eq!(
        task_1594::feature_pages_peaks(&[5, 2, 2, 8, 9, 1, 14, 1, 1]),
        vec![160u32, 256u32, 288u32, 448u32]
    );
}
