use task_2953::*;

#[test]
fn public_examples() {
    assert_eq!(journal_replay_bounds(&[]), None);
    assert_eq!(journal_replay_bounds(&[4]), Some((4u32, 4u32)));
}
