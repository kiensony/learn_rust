use task_1298::*;

#[test]
fn transfer_case() {
    assert_eq!(
        explicit_levels_rolling_load("--level=3 data --level=9 --level=3 data --level=9", 3),
        vec![15u64, 21u64]
    );
}
