use task_2775::*;

#[test]
fn public_examples() {
    assert_eq!(repeated_keys_changes(""), vec![]);
    assert_eq!(repeated_keys_changes("a=1"), vec![]);
}
