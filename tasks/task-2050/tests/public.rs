use task_2050::*;

#[test]
fn public_examples() {
    assert_eq!(
        polled_costs_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[2, 2, 2, 2, 3, 4, 0, 3, 1]
        ),
        Some(9)
    );
    assert_eq!(polled_costs_replay_mismatch(&[], &[]), None);
}
