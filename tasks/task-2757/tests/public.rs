use task_2757::*;

#[test]
fn public_examples() {
    assert_eq!(parsed_values_distinct(""), vec![]);
    assert_eq!(parsed_values_distinct("a=1"), vec![1u32]);
}
