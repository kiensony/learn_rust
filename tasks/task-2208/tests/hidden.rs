use task_2208::*;

#[test]
fn edge_cases() {
    assert_eq!(
        borrowed_prefixes_steps("a\nblue\nblue\r\nz\n"),
        vec![3u32, 0u32, 3u32]
    );
    assert_eq!(
        borrowed_prefixes_steps("   x\té\n\ncat\n"),
        vec![1u32, 1u32]
    );
    assert_eq!(
        borrowed_prefixes_steps("nine\nq\nthree\nx\n"),
        vec![3u32, 4u32, 4u32]
    );
    assert_eq!(
        borrowed_prefixes_steps("a b c\n\r\nup\tdown"),
        vec![0u32, 0u32, 1u32, 2u32]
    );
    assert_eq!(borrowed_prefixes_steps("\n\nzz\n"), vec![]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2208::TRACK_PROOF, 0x52540000 | 2208);
}
