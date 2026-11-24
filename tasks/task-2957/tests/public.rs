use task_2957::*;

#[test]
fn public_examples() {
    assert_eq!(journal_replay_distinct(&[]), vec![]);
    assert_eq!(journal_replay_distinct(&[4]), vec![4u32]);
}
