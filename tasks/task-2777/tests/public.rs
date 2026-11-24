use task_2777::*;

#[test]
fn public_examples() {
    assert_eq!(repeated_keys_distinct(""), vec![]);
    assert_eq!(repeated_keys_distinct("a=1"), vec![]);
}
