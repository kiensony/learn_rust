use task_1337::*;

#[test]
fn public_examples() {
    assert_eq!(field_indexes_distinct(""), vec![]);
    assert_eq!(field_indexes_distinct("1"), vec![]);
}
