use task_2204::*;

#[test]
fn edge_cases() {
    assert_eq!(
        borrowed_prefixes_peaks("a\nblue\nblue\r\nz\n"),
        vec![1u32, 4u32]
    );
    assert_eq!(
        borrowed_prefixes_peaks("   x\té\n\ncat\n"),
        vec![1u32, 2u32, 3u32]
    );
    assert_eq!(
        borrowed_prefixes_peaks("nine\nq\nthree\nx\n"),
        vec![4u32, 5u32]
    );
    assert_eq!(
        borrowed_prefixes_peaks("a b c\n\r\nup\tdown"),
        vec![1u32, 2u32, 4u32]
    );
    assert_eq!(borrowed_prefixes_peaks("\n\nzz\n"), vec![2u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2204::TRACK_PROOF, 0x52540000 | 2204);
}
