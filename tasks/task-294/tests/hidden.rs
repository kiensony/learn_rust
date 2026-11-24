use task_294::*;

#[test]
fn edge_cases() {
    assert_eq!(read_a_heading_plus(""), "");
    assert_eq!(read_a_heading_plus("+"), "");
    assert_eq!(read_a_heading_plus("++"), "");
    assert_eq!(read_a_heading_plus(" café "), " café ");
    assert_eq!(read_a_heading_plus("a+b"), "a");
    assert_eq!(read_a_heading_plus("🐈+猫"), "🐈");
}

#[test]
fn submission_marker() {
    assert_eq!(task_294::TRACK_PROOF, 0x52540000 | 294);
}
