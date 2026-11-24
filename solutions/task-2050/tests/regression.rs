use task_2050::*;

#[test]
fn transfer_case() {
    assert_eq!(
        polled_costs_replay_mismatch(
            &[
                3, 6, 9, 12, 21, 15, 0, 26, 8, 31, 31, 8, 26, 0, 15, 21, 12, 9, 6, 3
            ],
            &[2, 2, 2, 2, 3, 4, 0, 3, 1, 5, 5, 1, 3, 0, 4, 3, 2, 2, 2, 2]
        ),
        None
    );
}
