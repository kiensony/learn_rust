use task_2955::*;

#[test]
fn public_examples() {
    assert_eq!(journal_replay_changes(&[]), vec![]);
    assert_eq!(journal_replay_changes(&[4]), vec![4u32]);
}
