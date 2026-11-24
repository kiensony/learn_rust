use task_2875::*;

#[test]
fn public_examples() {
    assert_eq!(flag_register_changes(&[]), vec![]);
    assert_eq!(flag_register_changes(&[4]), vec![]);
}
