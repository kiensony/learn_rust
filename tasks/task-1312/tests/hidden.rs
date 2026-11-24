use task_1312::*;

#[test]
fn edge_cases() {
    assert_eq!(nibble_fields_total("4,9,2,2,0"), 0);
    assert_eq!(nibble_fields_total(" , 12,xyz,-1,,255,256,"), 15);
    assert_eq!(nibble_fields_total("0,0,0"), 0);
    assert_eq!(nibble_fields_total("12,3,12,4,5"), 0);
    assert_eq!(nibble_fields_total("1, 02 ,3"), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1312::TRACK_PROOF, 0x52540000 | 1312);
}
