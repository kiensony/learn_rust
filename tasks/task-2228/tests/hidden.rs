use task_2228::*;

#[test]
fn edge_cases() {
    assert_eq!(
        borrowed_scalars_steps("a\nblue\nblue\r\nz\n"),
        vec![
            1u32, 1u32, 1u32, 1u32, 1u32, 1u32, 1u32, 1u32, 1u32, 1u32, 1u32, 1u32, 1u32, 1u32
        ]
    );
    assert_eq!(
        borrowed_scalars_steps("   x\té\n\ncat\n"),
        vec![
            1u32, 1u32, 1u32, 1u32, 1u32, 2u32, 1u32, 1u32, 1u32, 1u32, 1u32
        ]
    );
    assert_eq!(
        borrowed_scalars_steps("nine\nq\nthree\nx\n"),
        vec![
            1u32, 1u32, 1u32, 1u32, 1u32, 1u32, 1u32, 1u32, 1u32, 1u32, 1u32, 1u32, 1u32, 1u32
        ]
    );
    assert_eq!(
        borrowed_scalars_steps("a b c\n\r\nup\tdown"),
        vec![
            1u32, 1u32, 1u32, 1u32, 1u32, 1u32, 1u32, 1u32, 1u32, 1u32, 1u32, 1u32, 1u32, 1u32
        ]
    );
    assert_eq!(
        borrowed_scalars_steps("\n\nzz\n"),
        vec![1u32, 1u32, 1u32, 1u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2228::TRACK_PROOF, 0x52540000 | 2228);
}
