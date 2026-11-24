use task_1205::*;

#[test]
fn edge_cases() {
    assert_eq!(
        line_byte_lengths_changes("a\nblue\nblue\r\nz\n"),
        vec![1u32, 4u32, 1u32]
    );
    assert_eq!(
        line_byte_lengths_changes("   x\té\n\ncat\n"),
        vec![7u32, 0u32, 3u32]
    );
    assert_eq!(
        line_byte_lengths_changes("nine\nq\nthree\nx\n"),
        vec![4u32, 1u32, 5u32, 1u32]
    );
    assert_eq!(
        line_byte_lengths_changes("a b c\n\r\nup\tdown"),
        vec![5u32, 0u32, 7u32]
    );
    assert_eq!(line_byte_lengths_changes("\n\nzz\n"), vec![0u32, 2u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1205::TRACK_PROOF, 0x52540000 | 1205);
}
