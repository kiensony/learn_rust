use task_2200::*;

#[test]
fn public_examples() {
    assert_eq!(
        duplicate_insertion_cost_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[1, 2, 3, 4, 5, 6, 7, 8, 9]
        ),
        Some(9)
    );
    assert_eq!(duplicate_insertion_cost_replay_mismatch(&[], &[]), None);
}
