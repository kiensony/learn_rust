use task_1322::*;

#[test]
fn edge_cases() {
    assert_eq!(flag_fields_total("4,9,2,2,0"), 9);
    assert_eq!(flag_fields_total(" , 12,xyz,-1,,255,256,"), 255);
    assert_eq!(flag_fields_total("0,0,0"), 0);
    assert_eq!(flag_fields_total("12,3,12,4,5"), 8);
    assert_eq!(flag_fields_total("1, 02 ,3"), 4);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1322::TRACK_PROOF, 0x52540000 | 1322);
}
