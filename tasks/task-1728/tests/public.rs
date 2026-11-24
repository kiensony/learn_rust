use task_1728::*;

#[test]
fn public_examples() {
    assert_eq!(boolean_macro_steps(&[]), vec![]);
    assert_eq!(boolean_macro_steps(&[4]), vec![]);
}
