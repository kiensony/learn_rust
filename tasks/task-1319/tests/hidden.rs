use task_1319::*;

#[test]
fn edge_cases() {
    assert_eq!(nibble_fields_longest_run("4,9,2,2,0"), 5);
    assert_eq!(nibble_fields_longest_run(" , 12,xyz,-1,,255,256,"), 1);
    assert_eq!(nibble_fields_longest_run("0,0,0"), 3);
    assert_eq!(nibble_fields_longest_run("12,3,12,4,5"), 5);
    assert_eq!(nibble_fields_longest_run("1, 02 ,3"), 3);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1319::TRACK_PROOF, 0x52540000 | 1319);
}
