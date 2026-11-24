use task_304::*;

#[test]
fn edge_cases() {
    assert_eq!(read_a_heading_bang(""), "");
    assert_eq!(read_a_heading_bang("!"), "");
    assert_eq!(read_a_heading_bang("!!"), "");
    assert_eq!(read_a_heading_bang(" café "), " café ");
    assert_eq!(read_a_heading_bang("a!b"), "a");
}

#[test]
fn submission_marker() {
    assert_eq!(task_304::TRACK_PROOF, 0x52540000 | 304);
}
