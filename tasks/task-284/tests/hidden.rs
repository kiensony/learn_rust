use task_284::*;

#[test]
fn edge_cases() {
    assert_eq!(read_a_heading_tilde(""), "");
    assert_eq!(read_a_heading_tilde("~"), "");
    assert_eq!(read_a_heading_tilde("~~"), "");
    assert_eq!(read_a_heading_tilde(" café "), " café ");
    assert_eq!(read_a_heading_tilde("a~b"), "a");
    assert_eq!(read_a_heading_tilde("🐈~猫"), "🐈");
}

#[test]
fn submission_marker() {
    assert_eq!(task_284::TRACK_PROOF, 0x52540000 | 284);
}
