use task_1346::*;

#[test]
fn edge_cases() {
    assert_eq!(decimal_fields_threshold_crossing("1", 0), Some((0, 1)));
    assert_eq!(
        decimal_fields_threshold_crossing("4,9,2,2,0", 1),
        Some((0, 4))
    );
    assert_eq!(
        decimal_fields_threshold_crossing(" , 12,xyz,-1,,255,256,", 255),
        Some((1, 255))
    );
    assert_eq!(decimal_fields_threshold_crossing("0,0,0", 256), None);
    assert_eq!(
        decimal_fields_threshold_crossing("12,3,12,4,5", 0),
        Some((0, 12))
    );
    assert_eq!(
        decimal_fields_threshold_crossing("1, 02 ,3", 1),
        Some((0, 1))
    );
    assert_eq!(decimal_fields_threshold_crossing("4,9,2,2,0", 10), None);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1346::TRACK_PROOF, 0x52540000 | 1346);
}
