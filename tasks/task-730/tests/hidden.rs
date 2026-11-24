use task_730::*;

#[test]
fn edge_cases() {
    assert_eq!(port("65535"), Some(65535));
    assert_eq!(port("65536"), None);
    assert_eq!(port("bad"), None);
    assert_eq!(port(""), None);
}

#[test]
fn submission_marker() {
    assert_eq!(task_730::TRACK_PROOF, 0x52540000 | 730);
}
