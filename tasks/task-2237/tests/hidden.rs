use task_2237::*;

#[test]
fn edge_cases() {
    assert_eq!(
        zero_copy_lines_distinct("a\nblue\nblue\r\nz\n"),
        vec![1u32, 4u32]
    );
    assert_eq!(
        zero_copy_lines_distinct("   x\té\n\ncat\n"),
        vec![0u32, 3u32, 7u32]
    );
    assert_eq!(
        zero_copy_lines_distinct("nine\nq\nthree\nx\n"),
        vec![1u32, 4u32, 5u32]
    );
    assert_eq!(
        zero_copy_lines_distinct("a b c\n\r\nup\tdown"),
        vec![0u32, 5u32, 7u32]
    );
    assert_eq!(zero_copy_lines_distinct("\n\nzz\n"), vec![0u32, 2u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2237::TRACK_PROOF, 0x52540000 | 2237);
}
