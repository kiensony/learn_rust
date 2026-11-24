use task_2210::*;

#[test]
fn edge_cases() {
    assert_eq!(
        borrowed_prefixes_reverse("a\nblue\nblue\r\nz\n"),
        vec![1u32, 4u32, 4u32, 1u32]
    );
    assert_eq!(
        borrowed_prefixes_reverse("   x\té\n\ncat\n"),
        vec![3u32, 2u32, 1u32]
    );
    assert_eq!(
        borrowed_prefixes_reverse("nine\nq\nthree\nx\n"),
        vec![1u32, 5u32, 1u32, 4u32]
    );
    assert_eq!(
        borrowed_prefixes_reverse("a b c\n\r\nup\tdown"),
        vec![4u32, 2u32, 1u32, 1u32, 1u32]
    );
    assert_eq!(borrowed_prefixes_reverse("\n\nzz\n"), vec![2u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2210::TRACK_PROOF, 0x52540000 | 2210);
}
