use task_1746::*;

#[test]
fn public_examples() {
    assert_eq!(
        nibble_macro_threshold_crossing(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 0),
        Some((0, 3))
    );
    assert_eq!(nibble_macro_threshold_crossing(&[], 0), None);
}
