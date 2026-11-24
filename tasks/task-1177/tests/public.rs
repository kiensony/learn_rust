use task_1177::*;

#[test]
fn public_examples() {
    assert_eq!(stored_key(&["red", "blue"], "blue"), Some("blue"));
    assert_eq!(stored_key(&["no"], "yes"), None);
}
