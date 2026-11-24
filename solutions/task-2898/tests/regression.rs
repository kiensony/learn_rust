use task_2898::*;

#[test]
fn transfer_case() {
    assert_eq!(
        flag_register_rolling_load(
            &[
                3, 6, 9, 12, 21, 15, 0, 26, 8, 31, 31, 8, 26, 0, 15, 21, 12, 9, 6, 3
            ],
            3
        ),
        vec![15u64, 13u64, 13u64, 16u64, 16u64, 13u64, 13u64, 15u64]
    );
}
