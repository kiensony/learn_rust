use task_071::*;

#[test]
fn edge_cases() {
    assert_eq!(ticket_mark('雪'), ('雪', false));
    assert_eq!(ticket_mark('\n'), ('\n', false));
    assert_eq!(ticket_mark('🦀'), ('🦀', false));
}

#[test]
fn submission_marker() {
    assert_eq!(task_071::TRACK_PROOF, 0x52540000 | 71);
}
