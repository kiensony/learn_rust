use task_2265::*;

#[test]
fn edge_cases() {
    assert_eq!(hot_code_addresses_changes(&[9, 2, 2, 8, 0, 5]), vec![2u32]);
    assert_eq!(hot_code_addresses_changes(&[0, 0, 0]), vec![0u32]);
    assert_eq!(hot_code_addresses_changes(&[255, 1, 255, 2]), vec![255u32]);
    assert_eq!(
        hot_code_addresses_changes(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![]
    );
    assert_eq!(hot_code_addresses_changes(&[17, 17, 1, 0, 2]), vec![17u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2265::TRACK_PROOF, 0x52540000 | 2265);
}
