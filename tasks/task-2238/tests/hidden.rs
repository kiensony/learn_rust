use task_2238::*;

#[test]
fn edge_cases() {
    assert_eq!(
        zero_copy_lines_steps("a\nblue\nblue\r\nz\n"),
        vec![3u32, 0u32, 3u32]
    );
    assert_eq!(zero_copy_lines_steps("   x\té\n\ncat\n"), vec![7u32, 3u32]);
    assert_eq!(
        zero_copy_lines_steps("nine\nq\nthree\nx\n"),
        vec![3u32, 4u32, 4u32]
    );
    assert_eq!(
        zero_copy_lines_steps("a b c\n\r\nup\tdown"),
        vec![5u32, 7u32]
    );
    assert_eq!(zero_copy_lines_steps("\n\nzz\n"), vec![0u32, 2u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2238::TRACK_PROOF, 0x52540000 | 2238);
}
