use task_1197::*;

#[test]
fn edge_cases() {
    assert_eq!(columns_of(&[]), Vec::<(&str, &str)>::new());
    assert_eq!(columns_of(&[":", ":"]), vec![("", ""), ("", "")]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1197::TRACK_PROOF, 0x52540000 | 1197);
}
