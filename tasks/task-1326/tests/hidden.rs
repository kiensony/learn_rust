use task_1326::*;

#[test]
fn edge_cases() {
    assert_eq!(flag_fields_descending_steps("4,9,2,2,0"), 0);
    assert_eq!(flag_fields_descending_steps(" , 12,xyz,-1,,255,256,"), 0);
    assert_eq!(flag_fields_descending_steps("0,0,0"), 0);
    assert_eq!(flag_fields_descending_steps("12,3,12,4,5"), 0);
    assert_eq!(flag_fields_descending_steps("1, 02 ,3"), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1326::TRACK_PROOF, 0x52540000 | 1326);
}
