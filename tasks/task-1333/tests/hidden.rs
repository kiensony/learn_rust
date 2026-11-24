use task_1333::*;

#[test]
fn edge_cases() {
    assert_eq!(field_indexes_bounds("4,9,2,2,0"), Some((4u32, 4u32)));
    assert_eq!(field_indexes_bounds(" , 12,xyz,-1,,255,256,"), None);
    assert_eq!(field_indexes_bounds("0,0,0"), Some((0u32, 2u32)));
    assert_eq!(field_indexes_bounds("12,3,12,4,5"), None);
    assert_eq!(field_indexes_bounds("1, 02 ,3"), None);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1333::TRACK_PROOF, 0x52540000 | 1333);
}
