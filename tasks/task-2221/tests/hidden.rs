use task_2221::*;

#[test]
fn edge_cases() {
    assert_eq!(
        borrowed_scalars_trace("a\nblue\nblue\r\nz\n"),
        vec![
            0u32, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32, 9u32, 10u32, 11u32, 12u32, 13u32,
            14u32
        ]
    );
    assert_eq!(
        borrowed_scalars_trace("   x\té\n\ncat\n"),
        vec![
            0u32, 1u32, 2u32, 3u32, 4u32, 5u32, 7u32, 8u32, 9u32, 10u32, 11u32, 12u32
        ]
    );
    assert_eq!(
        borrowed_scalars_trace("nine\nq\nthree\nx\n"),
        vec![
            0u32, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32, 9u32, 10u32, 11u32, 12u32, 13u32,
            14u32
        ]
    );
    assert_eq!(
        borrowed_scalars_trace("a b c\n\r\nup\tdown"),
        vec![
            0u32, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32, 9u32, 10u32, 11u32, 12u32, 13u32,
            14u32
        ]
    );
    assert_eq!(
        borrowed_scalars_trace("\n\nzz\n"),
        vec![0u32, 1u32, 2u32, 3u32, 4u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2221::TRACK_PROOF, 0x52540000 | 2221);
}
