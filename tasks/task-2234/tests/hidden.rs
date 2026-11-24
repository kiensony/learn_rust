use task_2234::*;

#[test]
fn edge_cases() {
    assert_eq!(
        zero_copy_lines_peaks("a\nblue\nblue\r\nz\n"),
        vec![1u32, 4u32]
    );
    assert_eq!(zero_copy_lines_peaks("   x\té\n\ncat\n"), vec![7u32]);
    assert_eq!(
        zero_copy_lines_peaks("nine\nq\nthree\nx\n"),
        vec![4u32, 5u32]
    );
    assert_eq!(
        zero_copy_lines_peaks("a b c\n\r\nup\tdown"),
        vec![5u32, 7u32]
    );
    assert_eq!(zero_copy_lines_peaks("\n\nzz\n"), vec![0u32, 2u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2234::TRACK_PROOF, 0x52540000 | 2234);
}
