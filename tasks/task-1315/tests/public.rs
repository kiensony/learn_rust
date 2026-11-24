use task_1315::*;

#[test]
fn public_examples() {
    assert_eq!(nibble_fields_changes(""), vec![]);
    assert_eq!(nibble_fields_changes("1"), vec![0u32]);
}
