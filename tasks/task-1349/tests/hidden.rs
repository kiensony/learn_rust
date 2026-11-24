use task_1349::*;

#[test]
fn edge_cases() {
    assert_eq!(field_indexes_frequent_readings("1", 0), vec![]);
    assert_eq!(field_indexes_frequent_readings("4,9,2,2,0", 1), vec![4u32]);
    assert_eq!(
        field_indexes_frequent_readings(" , 12,xyz,-1,,255,256,", 2),
        vec![]
    );
    assert_eq!(
        field_indexes_frequent_readings("0,0,0", 3),
        vec![0u32, 1u32, 2u32]
    );
    assert_eq!(field_indexes_frequent_readings("12,3,12,4,5", 256), vec![]);
    assert_eq!(field_indexes_frequent_readings("1, 02 ,3", 0), vec![]);
    assert_eq!(field_indexes_frequent_readings("0,0,0", 1), vec![0u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1349::TRACK_PROOF, 0x52540000 | 1349);
}
