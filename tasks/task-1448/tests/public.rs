use task_1448::*;

#[test]
fn public_examples() {
    assert_eq!(
        remaining_budget_rolling_load(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 1),
        vec![
            1u64, 52u64, 103u64, 154u64, 307u64, 205u64, 0u64, 392u64, 86u64, 477u64
        ]
    );
    assert_eq!(remaining_budget_rolling_load(&[], 0), vec![]);
}
