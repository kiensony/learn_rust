use task_2198::*;

#[test]
fn transfer_case() {
    assert_eq!(
        prefix_comparison_budget_rolling_load(
            &[
                3, 6, 9, 12, 21, 15, 0, 26, 8, 31, 31, 8, 26, 0, 15, 21, 12, 9, 6, 3
            ],
            3
        ),
        vec![
            3u64, 6u64, 9u64, 12u64, 15u64, 18u64, 21u64, 24u64, 27u64, 30u64, 33u64, 36u64, 39u64,
            42u64, 45u64, 48u64, 51u64, 54u64
        ]
    );
}
