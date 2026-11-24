use task_1224::*;

#[test]
fn public_examples() {
    assert_eq!(delimited_fields_peaks(&[]), vec![]);
    assert_eq!(delimited_fields_peaks(&[4]), vec![1u32]);
}
