use task_2249::*;

#[test]
fn edge_cases() {
    assert_eq!(zero_copy_lines_frequent_readings("a", 0), vec![]);
    assert_eq!(
        zero_copy_lines_frequent_readings("a\nblue\nblue\r\nz\n", 1),
        vec![1u32]
    );
    assert_eq!(
        zero_copy_lines_frequent_readings("   x\té\n\ncat\n", 2),
        vec![7u32, 0u32]
    );
    assert_eq!(
        zero_copy_lines_frequent_readings("nine\nq\nthree\nx\n", 3),
        vec![1u32, 4u32, 5u32]
    );
    assert_eq!(
        zero_copy_lines_frequent_readings("a b c\n\r\nup\tdown", 256),
        vec![5u32, 0u32, 7u32]
    );
    assert_eq!(zero_copy_lines_frequent_readings("\n\nzz\n", 0), vec![]);
    assert_eq!(
        zero_copy_lines_frequent_readings("nine\nq\nthree\nx\n", 1),
        vec![1u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2249::TRACK_PROOF, 0x52540000 | 2249);
}
