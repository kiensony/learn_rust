use task_1765::*;

#[test]
fn edge_cases() {
    assert_eq!(
        generated_offset_changes(&[9, 2, 2, 8, 0, 5]),
        vec![16u32, 9u32, 15u32, 7u32, 12u32]
    );
    assert_eq!(generated_offset_changes(&[0, 0, 0]), vec![7u32]);
    assert_eq!(
        generated_offset_changes(&[255, 1, 255, 2]),
        vec![262u32, 8u32, 262u32, 9u32]
    );
    assert_eq!(
        generated_offset_changes(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            10u32, 13u32, 16u32, 19u32, 28u32, 22u32, 7u32, 33u32, 15u32, 38u32
        ]
    );
    assert_eq!(
        generated_offset_changes(&[17, 17, 1, 0, 2]),
        vec![24u32, 8u32, 7u32, 9u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1765::TRACK_PROOF, 0x52540000 | 1765);
}
