use task_1669::*;

#[test]
fn edge_cases() {
    assert_eq!(minor_version_longest_run("1.0.0 1.2.0 2.0.0"), 1);
    assert_eq!(minor_version_longest_run("1 3.1.8 bad 3.1.8"), 2);
    assert_eq!(minor_version_longest_run("0.255.3 1.0.9 256.1.1"), 1);
    assert_eq!(minor_version_longest_run("12.4.8 2.9.3 12.4.8"), 1);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1669::TRACK_PROOF, 0x52540000 | 1669);
}
