use task_1304::*;

#[test]
fn public_examples() {
    assert_eq!(decimal_fields_peaks(""), vec![]);
    assert_eq!(decimal_fields_peaks("1"), vec![1u32]);
}
