use task_2246::*;

#[test]
fn edge_cases() {
    assert_eq!(borrowed_prefixes_threshold_crossing("a", 0), Some((0, 1)));
    assert_eq!(
        borrowed_prefixes_threshold_crossing("a\nblue\nblue\r\nz\n", 1),
        Some((0, 1))
    );
    assert_eq!(
        borrowed_prefixes_threshold_crossing("   x\té\n\ncat\n", 3),
        Some((2, 3))
    );
    assert_eq!(
        borrowed_prefixes_threshold_crossing("nine\nq\nthree\nx\n", 256),
        None
    );
    assert_eq!(
        borrowed_prefixes_threshold_crossing("a b c\n\r\nup\tdown", 0),
        Some((0, 1))
    );
    assert_eq!(
        borrowed_prefixes_threshold_crossing("\n\nzz\n", 1),
        Some((0, 2))
    );
    assert_eq!(
        borrowed_prefixes_threshold_crossing("a b c\n\r\nup\tdown", 5),
        None
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2246::TRACK_PROOF, 0x52540000 | 2246);
}
