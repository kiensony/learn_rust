use task_2233::*;

#[test]
fn edge_cases() {
    assert_eq!(
        zero_copy_lines_bounds("a\nblue\nblue\r\nz\n"),
        Some((1u32, 4u32))
    );
    assert_eq!(
        zero_copy_lines_bounds("   x\té\n\ncat\n"),
        Some((0u32, 7u32))
    );
    assert_eq!(
        zero_copy_lines_bounds("nine\nq\nthree\nx\n"),
        Some((1u32, 5u32))
    );
    assert_eq!(
        zero_copy_lines_bounds("a b c\n\r\nup\tdown"),
        Some((0u32, 7u32))
    );
    assert_eq!(zero_copy_lines_bounds("\n\nzz\n"), Some((0u32, 2u32)));
}

#[test]
fn submission_marker() {
    assert_eq!(task_2233::TRACK_PROOF, 0x52540000 | 2233);
}
