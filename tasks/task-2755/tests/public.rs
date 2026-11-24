use task_2755::*;

#[test]
fn public_examples() {
    assert_eq!(parsed_values_changes(""), vec![]);
    assert_eq!(parsed_values_changes("a=1"), vec![1u32]);
}
