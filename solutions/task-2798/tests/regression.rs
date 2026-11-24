use task_2798::*;

#[test]
fn transfer_case() {
    assert_eq!(
        repeated_keys_rolling_load("b=02;a=9;a=9;b=4 b=02;a=9;a=9;b=4", 3),
        vec![27u64, 22u64]
    );
}
