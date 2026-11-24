use task_1228::*;

#[test]
fn edge_cases() {
    assert_eq!(delimited_fields_steps(&[9, 2, 2, 8, 0, 5]), vec![3u32]);
    assert_eq!(delimited_fields_steps(&[0, 0, 0]), vec![0u32, 0u32]);
    assert_eq!(delimited_fields_steps(&[255, 1, 255, 2]), vec![]);
    assert_eq!(
        delimited_fields_steps(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![3u32]
    );
    assert_eq!(delimited_fields_steps(&[17, 17, 1, 0, 2]), vec![2u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1228::TRACK_PROOF, 0x52540000 | 1228);
}
