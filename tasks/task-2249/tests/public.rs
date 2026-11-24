use task_2249::*;

#[test]
fn public_examples() {
    assert_eq!(
        zero_copy_lines_frequent_readings("nine\nq\nthree\nx\n", 3),
        vec![1u32, 4u32, 5u32]
    );
    assert_eq!(zero_copy_lines_frequent_readings("", 0), vec![]);
}
