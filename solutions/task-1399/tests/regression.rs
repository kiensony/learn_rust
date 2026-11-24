use task_1399::*;

#[test]
fn transfer_case() {
    assert_eq!(
        first_frame_byte_frequent_readings(
            &[2, 7, 3, 1, 4, 0, 3, 9, 9, 2, 2, 9, 9, 3, 0, 4, 1, 3, 7, 2],
            2
        ),
        vec![0u32, 9u32]
    );
}
