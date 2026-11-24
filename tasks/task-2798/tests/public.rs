use task_2798::*;

#[test]
fn public_examples() {
    assert_eq!(
        repeated_keys_rolling_load("b=02;a=9;a=9;b=4", 1),
        vec![9u64, 4u64]
    );
    assert_eq!(repeated_keys_rolling_load("", 0), vec![]);
}
