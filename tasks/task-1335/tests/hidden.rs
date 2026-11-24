use task_1335::*;

#[test]
fn edge_cases() {
    assert_eq!(field_indexes_changes("4,9,2,2,0"), vec![4u32]);
    assert_eq!(field_indexes_changes(" , 12,xyz,-1,,255,256,"), vec![]);
    assert_eq!(field_indexes_changes("0,0,0"), vec![0u32, 1u32, 2u32]);
    assert_eq!(field_indexes_changes("12,3,12,4,5"), vec![]);
    assert_eq!(field_indexes_changes("1, 02 ,3"), vec![]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1335::TRACK_PROOF, 0x52540000 | 1335);
}
