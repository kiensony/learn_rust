use task_1791::*;

#[test]
fn edge_cases() {
    assert_eq!(
        token_offset_trace(&[9, 2, 2, 8, 0, 5]),
        vec![14u32, 7u32, 7u32, 13u32, 5u32, 10u32]
    );
    assert_eq!(token_offset_trace(&[0, 0, 0]), vec![5u32, 5u32, 5u32]);
    assert_eq!(
        token_offset_trace(&[255, 1, 255, 2]),
        vec![260u32, 6u32, 260u32, 7u32]
    );
    assert_eq!(
        token_offset_trace(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            8u32, 11u32, 14u32, 17u32, 26u32, 20u32, 5u32, 31u32, 13u32, 36u32
        ]
    );
    assert_eq!(
        token_offset_trace(&[17, 17, 1, 0, 2]),
        vec![22u32, 22u32, 6u32, 5u32, 7u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1791::TRACK_PROOF, 0x52540000 | 1791);
}
