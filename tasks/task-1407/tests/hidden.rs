use task_1407::*;

#[test]
fn edge_cases() {
    assert_eq!(
        millisecond_remainder_distinct(&[9, 2, 2, 8, 0, 5]),
        vec![0u32, 34u32, 85u32, 136u32, 153u32]
    );
    assert_eq!(millisecond_remainder_distinct(&[0, 0, 0]), vec![0u32]);
    assert_eq!(
        millisecond_remainder_distinct(&[255, 1, 255, 2]),
        vec![17u32, 34u32, 335u32]
    );
    assert_eq!(
        millisecond_remainder_distinct(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            0u32, 51u32, 102u32, 136u32, 153u32, 204u32, 255u32, 357u32, 442u32, 527u32
        ]
    );
    assert_eq!(
        millisecond_remainder_distinct(&[17, 17, 1, 0, 2]),
        vec![0u32, 17u32, 34u32, 289u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1407::TRACK_PROOF, 0x52540000 | 1407);
}
