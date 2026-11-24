use task_2223::*;

#[test]
fn edge_cases() {
    assert_eq!(
        borrowed_scalars_bounds("a\nblue\nblue\r\nz\n"),
        Some((0u32, 14u32))
    );
    assert_eq!(
        borrowed_scalars_bounds("   x\té\n\ncat\n"),
        Some((0u32, 12u32))
    );
    assert_eq!(
        borrowed_scalars_bounds("nine\nq\nthree\nx\n"),
        Some((0u32, 14u32))
    );
    assert_eq!(
        borrowed_scalars_bounds("a b c\n\r\nup\tdown"),
        Some((0u32, 14u32))
    );
    assert_eq!(borrowed_scalars_bounds("\n\nzz\n"), Some((0u32, 4u32)));
}

#[test]
fn submission_marker() {
    assert_eq!(task_2223::TRACK_PROOF, 0x52540000 | 2223);
}
