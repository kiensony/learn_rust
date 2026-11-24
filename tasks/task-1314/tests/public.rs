use task_1314::*;

#[test]
fn public_examples() {
    assert_eq!(nibble_fields_peaks(""), vec![]);
    assert_eq!(nibble_fields_peaks("1"), vec![0u32]);
}
