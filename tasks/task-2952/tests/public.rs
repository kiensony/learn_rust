use task_2952::*;

#[test]
fn public_examples() {
    assert_eq!(journal_replay_total(&[]), 0);
    assert_eq!(journal_replay_total(&[4]), 4);
}
