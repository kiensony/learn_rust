use task_1770::*;

#[test]
fn edge_cases() {
    assert_eq!(
        generated_offset_reverse(&[9, 2, 2, 8, 0, 5]),
        vec![12u32, 7u32, 15u32, 9u32, 9u32, 16u32]
    );
    assert_eq!(generated_offset_reverse(&[0, 0, 0]), vec![7u32, 7u32, 7u32]);
    assert_eq!(
        generated_offset_reverse(&[255, 1, 255, 2]),
        vec![9u32, 262u32, 8u32, 262u32]
    );
    assert_eq!(
        generated_offset_reverse(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            38u32, 15u32, 33u32, 7u32, 22u32, 28u32, 19u32, 16u32, 13u32, 10u32
        ]
    );
    assert_eq!(
        generated_offset_reverse(&[17, 17, 1, 0, 2]),
        vec![9u32, 7u32, 8u32, 24u32, 24u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1770::TRACK_PROOF, 0x52540000 | 1770);
}
