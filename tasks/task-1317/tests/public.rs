use task_1317::*;

#[test]
fn public_examples() {
    assert_eq!(nibble_fields_distinct(""), vec![]);
    assert_eq!(nibble_fields_distinct("1"), vec![0u32]);
}
