use task_1332::*;

#[test]
fn edge_cases() {
    assert_eq!(field_indexes_total("4,9,2,2,0"), 4);
    assert_eq!(field_indexes_total(" , 12,xyz,-1,,255,256,"), 0);
    assert_eq!(field_indexes_total("0,0,0"), 3);
    assert_eq!(field_indexes_total("12,3,12,4,5"), 0);
    assert_eq!(field_indexes_total("1, 02 ,3"), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1332::TRACK_PROOF, 0x52540000 | 1332);
}
