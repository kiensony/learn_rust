use task_2772::*;

#[test]
fn public_examples() {
    assert_eq!(repeated_keys_total(""), 0);
    assert_eq!(repeated_keys_total("a=1"), 0);
}
