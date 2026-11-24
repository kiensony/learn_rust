use task_2218::*;

#[test]
fn edge_cases() {
    assert_eq!(
        borrowed_offsets_steps("a\nblue\nblue\r\nz\n"),
        vec![5u32, 6u32, 2u32]
    );
    assert_eq!(borrowed_offsets_steps("   x\té\n\ncat\n"), vec![1u32, 4u32]);
    assert_eq!(
        borrowed_offsets_steps("nine\nq\nthree\nx\n"),
        vec![2u32, 6u32, 2u32]
    );
    assert_eq!(borrowed_offsets_steps("a b c\n\r\nup\tdown"), vec![2u32]);
    assert_eq!(borrowed_offsets_steps("\n\nzz\n"), vec![1u32, 3u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2218::TRACK_PROOF, 0x52540000 | 2218);
}
