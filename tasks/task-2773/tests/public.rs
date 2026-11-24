use task_2773::*;

#[test]
fn public_examples() {
    assert_eq!(repeated_keys_bounds(""), None);
    assert_eq!(repeated_keys_bounds("a=1"), None);
}
