use task_274::*;

#[test]
fn edge_cases() {
    assert_eq!(read_a_heading_star(""), "");
    assert_eq!(read_a_heading_star("*"), "");
    assert_eq!(read_a_heading_star("**"), "");
    assert_eq!(read_a_heading_star(" café "), " café ");
    assert_eq!(read_a_heading_star("a*b"), "a");
    assert_eq!(read_a_heading_star("🐈*猫"), "🐈");
}

#[test]
fn submission_marker() {
    assert_eq!(task_274::TRACK_PROOF, 0x52540000 | 274);
}
