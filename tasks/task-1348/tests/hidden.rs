use task_1348::*;

#[test]
fn edge_cases() {
    assert_eq!(flag_fields_rolling_load("1", 1), vec![1u64]);
    assert_eq!(flag_fields_rolling_load("4,9,2,2,0", 2), vec![]);
    assert_eq!(
        flag_fields_rolling_load(" , 12,xyz,-1,,255,256,", 0),
        vec![]
    );
    assert_eq!(flag_fields_rolling_load("0,0,0", 100), vec![]);
    assert_eq!(flag_fields_rolling_load("12,3,12,4,5", 3), vec![]);
    assert_eq!(flag_fields_rolling_load("1, 02 ,3", 1), vec![1u64, 3u64]);
    assert_eq!(flag_fields_rolling_load("12,3,12,4,5", 2), vec![8u64]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1348::TRACK_PROOF, 0x52540000 | 1348);
}
