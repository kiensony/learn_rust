use task_1310::*;

#[test]
fn edge_cases() {
    assert_eq!(
        decimal_fields_reverse("4,9,2,2,0"),
        vec![0u32, 2u32, 2u32, 9u32, 4u32]
    );
    assert_eq!(
        decimal_fields_reverse(" , 12,xyz,-1,,255,256,"),
        vec![255u32, 12u32]
    );
    assert_eq!(decimal_fields_reverse("0,0,0"), vec![0u32, 0u32, 0u32]);
    assert_eq!(
        decimal_fields_reverse("12,3,12,4,5"),
        vec![5u32, 4u32, 12u32, 3u32, 12u32]
    );
    assert_eq!(decimal_fields_reverse("1, 02 ,3"), vec![3u32, 2u32, 1u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1310::TRACK_PROOF, 0x52540000 | 1310);
}
