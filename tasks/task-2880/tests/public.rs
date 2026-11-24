use task_2880::*;

#[test]
fn public_examples() {
    assert_eq!(flag_register_reverse(&[]), vec![]);
    assert_eq!(flag_register_reverse(&[4]), vec![]);
}
