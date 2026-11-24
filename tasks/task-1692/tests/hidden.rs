use task_1692::*;

#[test]
fn edge_cases() {
    assert_eq!(release_line_total("1.0.0 1.2.0 2.0.0"), 1026);
    assert_eq!(release_line_total("1 3.1.8 bad 3.1.8"), 1538);
    assert_eq!(release_line_total("0.255.3 1.0.9 256.1.1"), 511);
    assert_eq!(release_line_total("12.4.8 2.9.3 12.4.8"), 6673);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1692::TRACK_PROOF, 0x52540000 | 1692);
}
