use task_2284::*;

#[test]
fn edge_cases() {
    assert_eq!(find_newlines(b"\n\n\n"), 3);
    assert_eq!(find_newlines(b"abc"), 0);
    assert_eq!(find_newlines(b"x\r\ny\n"), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2284::TRACK_PROOF, 0x52540000 | 2284);
}
