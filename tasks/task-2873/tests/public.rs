use task_2873::*;

#[test]
fn public_examples() {
    assert_eq!(flag_register_bounds(&[]), None);
    assert_eq!(flag_register_bounds(&[4]), None);
}
