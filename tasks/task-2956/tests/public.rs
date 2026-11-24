use task_2956::*;

#[test]
fn public_examples() {
    assert_eq!(journal_replay_descending_steps(&[]), 0);
    assert_eq!(journal_replay_descending_steps(&[4]), 0);
}
