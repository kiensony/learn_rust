use task_2751::*;

#[test]
fn public_examples() {
    assert_eq!(parsed_values_trace(""), vec![]);
    assert_eq!(parsed_values_trace("a=1"), vec![1u32]);
}
