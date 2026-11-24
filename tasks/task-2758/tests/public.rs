use task_2758::*;

#[test]
fn public_examples() {
    assert_eq!(parsed_values_steps(""), vec![]);
    assert_eq!(parsed_values_steps("a=1"), vec![]);
}
