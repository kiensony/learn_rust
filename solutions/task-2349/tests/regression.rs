use task_2349::*;

#[test]
fn transfer_case() {
    assert_eq!(
        batch_variation_frequent_readings(
            &[
                3, 6, 9, 12, 21, 15, 0, 26, 8, 31, 31, 8, 26, 0, 15, 21, 12, 9, 6, 3
            ],
            2
        ),
        vec![9u32, 26u32]
    );
}
