use task_2954::*;

#[test]
fn public_examples() {
    assert_eq!(journal_replay_peaks(&[]), vec![]);
    assert_eq!(journal_replay_peaks(&[4]), vec![4u32]);
}
