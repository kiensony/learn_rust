use task_2225::*;

#[test]
fn public_examples() {
    assert_eq!(borrowed_scalars_changes(""), vec![]);
    assert_eq!(borrowed_scalars_changes("a"), vec![0u32]);
}
