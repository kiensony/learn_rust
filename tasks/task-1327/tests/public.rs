use task_1327::*;

#[test]
fn public_examples() {
    assert_eq!(flag_fields_distinct(""), vec![]);
    assert_eq!(flag_fields_distinct("1"), vec![1u32]);
}
