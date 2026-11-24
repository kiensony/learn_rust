use task_2201::*;

#[test]
fn edge_cases() {
    assert_eq!(
        borrowed_prefixes_trace("a\nblue\nblue\r\nz\n"),
        vec![1u32, 4u32, 4u32, 1u32]
    );
    assert_eq!(
        borrowed_prefixes_trace("   x\té\n\ncat\n"),
        vec![1u32, 2u32, 3u32]
    );
    assert_eq!(
        borrowed_prefixes_trace("nine\nq\nthree\nx\n"),
        vec![4u32, 1u32, 5u32, 1u32]
    );
    assert_eq!(
        borrowed_prefixes_trace("a b c\n\r\nup\tdown"),
        vec![1u32, 1u32, 1u32, 2u32, 4u32]
    );
    assert_eq!(borrowed_prefixes_trace("\n\nzz\n"), vec![2u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2201::TRACK_PROOF, 0x52540000 | 2201);
}
