use task_1204::*;

#[test]
fn edge_cases() {
    assert_eq!(
        line_byte_lengths_peaks("a\nblue\nblue\r\nz\n"),
        vec![1u32, 4u32]
    );
    assert_eq!(line_byte_lengths_peaks("   x\té\n\ncat\n"), vec![7u32]);
    assert_eq!(
        line_byte_lengths_peaks("nine\nq\nthree\nx\n"),
        vec![4u32, 5u32]
    );
    assert_eq!(
        line_byte_lengths_peaks("a b c\n\r\nup\tdown"),
        vec![5u32, 7u32]
    );
    assert_eq!(line_byte_lengths_peaks("\n\nzz\n"), vec![0u32, 2u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1204::TRACK_PROOF, 0x52540000 | 1204);
}
