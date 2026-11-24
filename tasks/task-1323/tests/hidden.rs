use task_1323::*;

#[test]
fn edge_cases() {
    assert_eq!(flag_fields_bounds("4,9,2,2,0"), Some((9u32, 9u32)));
    assert_eq!(
        flag_fields_bounds(" , 12,xyz,-1,,255,256,"),
        Some((255u32, 255u32))
    );
    assert_eq!(flag_fields_bounds("0,0,0"), None);
    assert_eq!(flag_fields_bounds("12,3,12,4,5"), Some((3u32, 5u32)));
    assert_eq!(flag_fields_bounds("1, 02 ,3"), Some((1u32, 3u32)));
}

#[test]
fn submission_marker() {
    assert_eq!(task_1323::TRACK_PROOF, 0x52540000 | 1323);
}
