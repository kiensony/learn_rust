use task_1793::*;

#[test]
fn edge_cases() {
    assert_eq!(
        token_offset_bounds(&[9, 2, 2, 8, 0, 5]),
        Some((5u32, 14u32))
    );
    assert_eq!(token_offset_bounds(&[0, 0, 0]), Some((5u32, 5u32)));
    assert_eq!(token_offset_bounds(&[255, 1, 255, 2]), Some((6u32, 260u32)));
    assert_eq!(
        token_offset_bounds(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        Some((5u32, 36u32))
    );
    assert_eq!(token_offset_bounds(&[17, 17, 1, 0, 2]), Some((5u32, 22u32)));
}

#[test]
fn submission_marker() {
    assert_eq!(task_1793::TRACK_PROOF, 0x52540000 | 1793);
}
