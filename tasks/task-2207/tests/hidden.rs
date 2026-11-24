use task_2207::*;

#[test]
fn edge_cases() {
    assert_eq!(
        borrowed_prefixes_distinct("a\nblue\nblue\r\nz\n"),
        vec![1u32, 4u32]
    );
    assert_eq!(
        borrowed_prefixes_distinct("   x\té\n\ncat\n"),
        vec![1u32, 2u32, 3u32]
    );
    assert_eq!(
        borrowed_prefixes_distinct("nine\nq\nthree\nx\n"),
        vec![1u32, 4u32, 5u32]
    );
    assert_eq!(
        borrowed_prefixes_distinct("a b c\n\r\nup\tdown"),
        vec![1u32, 2u32, 4u32]
    );
    assert_eq!(borrowed_prefixes_distinct("\n\nzz\n"), vec![2u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2207::TRACK_PROOF, 0x52540000 | 2207);
}
