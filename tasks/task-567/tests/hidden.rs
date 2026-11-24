use task_567::*;

#[test]
fn edge_cases() {
    assert_eq!(left_caption("", "x"), "");
    assert_eq!(left_caption("雪", ""), "雪");
    assert_eq!(left_caption("  ", "short"), "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_567::TRACK_PROOF, 0x52540000 | 567);
}
