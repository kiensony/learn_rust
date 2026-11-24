use task_1399::*;

#[test]
fn public_examples() {
    assert_eq!(
        first_frame_byte_frequent_readings(&[2, 7, 3, 1, 4, 0, 3, 9, 9, 2], 3),
        vec![7u32, 4u32, 0u32]
    );
    assert_eq!(first_frame_byte_frequent_readings(&[], 0), vec![]);
}
