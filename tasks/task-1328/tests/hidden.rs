use task_1328::*;

#[test]
fn edge_cases() {
    assert_eq!(flag_fields_steps("4,9,2,2,0"), vec![]);
    assert_eq!(flag_fields_steps(" , 12,xyz,-1,,255,256,"), vec![]);
    assert_eq!(flag_fields_steps("0,0,0"), vec![]);
    assert_eq!(flag_fields_steps("12,3,12,4,5"), vec![2u32]);
    assert_eq!(flag_fields_steps("1, 02 ,3"), vec![2u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1328::TRACK_PROOF, 0x52540000 | 1328);
}
