use task_2205::*;

#[test]
fn public_examples() {
    assert_eq!(borrowed_prefixes_changes(""), vec![]);
    assert_eq!(borrowed_prefixes_changes("a"), vec![1u32]);
}
