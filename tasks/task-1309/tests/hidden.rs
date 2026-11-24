use task_1309::*;

#[test]
fn edge_cases() {
    assert_eq!(decimal_fields_longest_run("4,9,2,2,0"), 2);
    assert_eq!(decimal_fields_longest_run(" , 12,xyz,-1,,255,256,"), 1);
    assert_eq!(decimal_fields_longest_run("0,0,0"), 3);
    assert_eq!(decimal_fields_longest_run("12,3,12,4,5"), 1);
    assert_eq!(decimal_fields_longest_run("1, 02 ,3"), 1);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1309::TRACK_PROOF, 0x52540000 | 1309);
}
