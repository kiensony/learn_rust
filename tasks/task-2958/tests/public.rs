use task_2958::*;

#[test]
fn public_examples() {
    assert_eq!(journal_replay_steps(&[]), vec![]);
    assert_eq!(journal_replay_steps(&[4]), vec![]);
}
