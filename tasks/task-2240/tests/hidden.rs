use task_2240::*;

#[test]
fn edge_cases() {
    assert_eq!(
        zero_copy_lines_reverse("a\nblue\nblue\r\nz\n"),
        vec![1u32, 4u32, 4u32, 1u32]
    );
    assert_eq!(
        zero_copy_lines_reverse("   x\té\n\ncat\n"),
        vec![3u32, 0u32, 7u32]
    );
    assert_eq!(
        zero_copy_lines_reverse("nine\nq\nthree\nx\n"),
        vec![1u32, 5u32, 1u32, 4u32]
    );
    assert_eq!(
        zero_copy_lines_reverse("a b c\n\r\nup\tdown"),
        vec![7u32, 0u32, 5u32]
    );
    assert_eq!(zero_copy_lines_reverse("\n\nzz\n"), vec![2u32, 0u32, 0u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2240::TRACK_PROOF, 0x52540000 | 2240);
}
