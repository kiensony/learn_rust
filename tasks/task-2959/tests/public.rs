use task_2959::*;

#[test]
fn public_examples() {
    assert_eq!(journal_replay_longest_run(&[]), 0);
    assert_eq!(journal_replay_longest_run(&[4]), 1);
}
