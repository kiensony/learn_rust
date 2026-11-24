use task_1338::*;

#[test]
fn edge_cases() {
    assert_eq!(field_indexes_steps("4,9,2,2,0"), vec![]);
    assert_eq!(field_indexes_steps(" , 12,xyz,-1,,255,256,"), vec![]);
    assert_eq!(field_indexes_steps("0,0,0"), vec![1u32, 1u32]);
    assert_eq!(field_indexes_steps("12,3,12,4,5"), vec![]);
    assert_eq!(field_indexes_steps("1, 02 ,3"), vec![]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1338::TRACK_PROOF, 0x52540000 | 1338);
}
