use task_1342::*;

#[test]
fn edge_cases() {
    assert_eq!(packed_booleans_total("4,9,2,2,0"), 5);
    assert_eq!(packed_booleans_total(" , 12,xyz,-1,,255,256,"), 10);
    assert_eq!(packed_booleans_total("0,0,0"), 0);
    assert_eq!(packed_booleans_total("12,3,12,4,5"), 9);
    assert_eq!(packed_booleans_total("1, 02 ,3"), 4);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1342::TRACK_PROOF, 0x52540000 | 1342);
}
