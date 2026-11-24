use task_2951::*;

#[test]
fn public_examples() {
    assert_eq!(journal_replay_trace(&[]), vec![]);
    assert_eq!(journal_replay_trace(&[4]), vec![4u32]);
}
