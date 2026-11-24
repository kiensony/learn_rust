use task_2249::*;

#[test]
fn transfer_case() {
    assert_eq!(
        zero_copy_lines_frequent_readings("nine\nq\nthree\nx\n nine\nq\nthree\nx\n", 2),
        vec![1u32, 5u32]
    );
}
