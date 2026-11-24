use task_2213::*;

#[test]
fn edge_cases() {
    assert_eq!(
        borrowed_offsets_bounds("a\nblue\nblue\r\nz\n"),
        Some((1u32, 14u32))
    );
    assert_eq!(
        borrowed_offsets_bounds("   x\té\n\ncat\n"),
        Some((7u32, 12u32))
    );
    assert_eq!(
        borrowed_offsets_bounds("nine\nq\nthree\nx\n"),
        Some((4u32, 14u32))
    );
    assert_eq!(
        borrowed_offsets_bounds("a b c\n\r\nup\tdown"),
        Some((5u32, 7u32))
    );
    assert_eq!(borrowed_offsets_bounds("\n\nzz\n"), Some((0u32, 4u32)));
}

#[test]
fn submission_marker() {
    assert_eq!(task_2213::TRACK_PROOF, 0x52540000 | 2213);
}
