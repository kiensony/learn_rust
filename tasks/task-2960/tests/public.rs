use task_2960::*;

#[test]
fn public_examples() {
    assert_eq!(journal_replay_reverse(&[]), vec![]);
    assert_eq!(journal_replay_reverse(&[4]), vec![4u32]);
}
