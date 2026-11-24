use task_1267::*;

#[test]
fn public_examples() {
    assert_eq!(positional_options_distinct(""), vec![]);
    assert_eq!(positional_options_distinct("only"), vec![4u32]);
}
