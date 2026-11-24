use task_1405::*;

#[test]
fn edge_cases() {
    assert_eq!(
        millisecond_remainder_changes(&[9, 2, 2, 8, 0, 5]),
        vec![153u32, 34u32, 136u32, 0u32, 85u32]
    );
    assert_eq!(millisecond_remainder_changes(&[0, 0, 0]), vec![0u32]);
    assert_eq!(
        millisecond_remainder_changes(&[255, 1, 255, 2]),
        vec![335u32, 17u32, 335u32, 34u32]
    );
    assert_eq!(
        millisecond_remainder_changes(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            51u32, 102u32, 153u32, 204u32, 357u32, 255u32, 0u32, 442u32, 136u32, 527u32
        ]
    );
    assert_eq!(
        millisecond_remainder_changes(&[17, 17, 1, 0, 2]),
        vec![289u32, 17u32, 0u32, 34u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1405::TRACK_PROOF, 0x52540000 | 1405);
}
