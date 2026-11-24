use task_1302::*;

#[test]
fn edge_cases() {
    assert_eq!(decimal_fields_total("4,9,2,2,0"), 17);
    assert_eq!(decimal_fields_total(" , 12,xyz,-1,,255,256,"), 267);
    assert_eq!(decimal_fields_total("0,0,0"), 0);
    assert_eq!(decimal_fields_total("12,3,12,4,5"), 36);
    assert_eq!(decimal_fields_total("1, 02 ,3"), 6);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1302::TRACK_PROOF, 0x52540000 | 1302);
}
