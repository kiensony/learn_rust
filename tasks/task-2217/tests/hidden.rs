use task_2217::*;

#[test]
fn edge_cases() {
    assert_eq!(
        borrowed_offsets_distinct("a\nblue\nblue\r\nz\n"),
        vec![1u32, 6u32, 12u32, 14u32]
    );
    assert_eq!(
        borrowed_offsets_distinct("   x\té\n\ncat\n"),
        vec![7u32, 8u32, 12u32]
    );
    assert_eq!(
        borrowed_offsets_distinct("nine\nq\nthree\nx\n"),
        vec![4u32, 6u32, 12u32, 14u32]
    );
    assert_eq!(
        borrowed_offsets_distinct("a b c\n\r\nup\tdown"),
        vec![5u32, 7u32]
    );
    assert_eq!(
        borrowed_offsets_distinct("\n\nzz\n"),
        vec![0u32, 1u32, 4u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2217::TRACK_PROOF, 0x52540000 | 2217);
}
