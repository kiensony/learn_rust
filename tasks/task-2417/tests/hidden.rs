use task_2417::*;

#[test]
fn edge_cases() {
    assert_eq!(
        word_alignment_distinct(&[9, 2, 2, 8, 0, 5]),
        vec![0u32, 4u32, 8u32, 12u32]
    );
    assert_eq!(word_alignment_distinct(&[0, 0, 0]), vec![0u32]);
    assert_eq!(
        word_alignment_distinct(&[255, 1, 255, 2]),
        vec![4u32, 256u32]
    );
    assert_eq!(
        word_alignment_distinct(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![0u32, 4u32, 8u32, 12u32, 16u32, 24u32, 28u32, 32u32]
    );
    assert_eq!(
        word_alignment_distinct(&[17, 17, 1, 0, 2]),
        vec![0u32, 4u32, 20u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2417::TRACK_PROOF, 0x52540000 | 2417);
}
