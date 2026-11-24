use task_1327::*;

#[test]
fn edge_cases() {
    assert_eq!(flag_fields_distinct("4,9,2,2,0"), vec![9u32]);
    assert_eq!(flag_fields_distinct(" , 12,xyz,-1,,255,256,"), vec![255u32]);
    assert_eq!(flag_fields_distinct("0,0,0"), vec![]);
    assert_eq!(flag_fields_distinct("12,3,12,4,5"), vec![3u32, 5u32]);
    assert_eq!(flag_fields_distinct("1, 02 ,3"), vec![1u32, 3u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1327::TRACK_PROOF, 0x52540000 | 1327);
}
