use task_1689::*;

#[test]
fn edge_cases() {
    assert_eq!(preview_version_longest_run("1.0.0 1.2.0 2.0.0"), 3);
    assert_eq!(preview_version_longest_run("1 3.1.8 bad 3.1.8"), 2);
    assert_eq!(preview_version_longest_run("0.255.3 1.0.9 256.1.1"), 1);
    assert_eq!(preview_version_longest_run("12.4.8 2.9.3 12.4.8"), 3);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1689::TRACK_PROOF, 0x52540000 | 1689);
}
