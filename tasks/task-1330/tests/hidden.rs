use task_1330::*;

#[test]
fn edge_cases() {
    assert_eq!(flag_fields_reverse("4,9,2,2,0"), vec![9u32]);
    assert_eq!(flag_fields_reverse(" , 12,xyz,-1,,255,256,"), vec![255u32]);
    assert_eq!(flag_fields_reverse("0,0,0"), vec![]);
    assert_eq!(flag_fields_reverse("12,3,12,4,5"), vec![5u32, 3u32]);
    assert_eq!(flag_fields_reverse("1, 02 ,3"), vec![3u32, 1u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1330::TRACK_PROOF, 0x52540000 | 1330);
}
