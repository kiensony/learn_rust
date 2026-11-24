use task_2207::*;

#[test]
fn public_examples() {
    assert_eq!(borrowed_prefixes_distinct(""), vec![]);
    assert_eq!(borrowed_prefixes_distinct("a"), vec![1u32]);
}
