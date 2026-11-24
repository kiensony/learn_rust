use task_2872::*;

#[test]
fn public_examples() {
    assert_eq!(flag_register_total(&[]), 0);
    assert_eq!(flag_register_total(&[4]), 0);
}
