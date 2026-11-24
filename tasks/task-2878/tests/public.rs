use task_2878::*;

#[test]
fn public_examples() {
    assert_eq!(flag_register_steps(&[]), vec![]);
    assert_eq!(flag_register_steps(&[4]), vec![]);
}
