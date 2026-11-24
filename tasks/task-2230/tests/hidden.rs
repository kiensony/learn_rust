use task_2230::*;

#[test]
fn edge_cases() {
    assert_eq!(
        borrowed_scalars_reverse("a\nblue\nblue\r\nz\n"),
        vec![
            14u32, 13u32, 12u32, 11u32, 10u32, 9u32, 8u32, 7u32, 6u32, 5u32, 4u32, 3u32, 2u32,
            1u32, 0u32
        ]
    );
    assert_eq!(
        borrowed_scalars_reverse("   x\té\n\ncat\n"),
        vec![
            12u32, 11u32, 10u32, 9u32, 8u32, 7u32, 5u32, 4u32, 3u32, 2u32, 1u32, 0u32
        ]
    );
    assert_eq!(
        borrowed_scalars_reverse("nine\nq\nthree\nx\n"),
        vec![
            14u32, 13u32, 12u32, 11u32, 10u32, 9u32, 8u32, 7u32, 6u32, 5u32, 4u32, 3u32, 2u32,
            1u32, 0u32
        ]
    );
    assert_eq!(
        borrowed_scalars_reverse("a b c\n\r\nup\tdown"),
        vec![
            14u32, 13u32, 12u32, 11u32, 10u32, 9u32, 8u32, 7u32, 6u32, 5u32, 4u32, 3u32, 2u32,
            1u32, 0u32
        ]
    );
    assert_eq!(
        borrowed_scalars_reverse("\n\nzz\n"),
        vec![4u32, 3u32, 2u32, 1u32, 0u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2230::TRACK_PROOF, 0x52540000 | 2230);
}
