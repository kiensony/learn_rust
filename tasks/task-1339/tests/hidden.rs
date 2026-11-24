use task_1339::*;

#[test]
fn edge_cases() {
    assert_eq!(field_indexes_longest_run("4,9,2,2,0"), 1);
    assert_eq!(field_indexes_longest_run(" , 12,xyz,-1,,255,256,"), 0);
    assert_eq!(field_indexes_longest_run("0,0,0"), 1);
    assert_eq!(field_indexes_longest_run("12,3,12,4,5"), 0);
    assert_eq!(field_indexes_longest_run("1, 02 ,3"), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1339::TRACK_PROOF, 0x52540000 | 1339);
}
