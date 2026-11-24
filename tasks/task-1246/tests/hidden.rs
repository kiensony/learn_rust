use task_1246::*;

#[test]
fn edge_cases() {
    assert_eq!(line_byte_lengths_threshold_crossing("a", 0), Some((0, 1)));
    assert_eq!(
        line_byte_lengths_threshold_crossing("a\nblue\nblue\r\nz\n", 1),
        Some((0, 1))
    );
    assert_eq!(
        line_byte_lengths_threshold_crossing("   x\té\n\ncat\n", 7),
        Some((0, 7))
    );
    assert_eq!(
        line_byte_lengths_threshold_crossing("nine\nq\nthree\nx\n", 256),
        None
    );
    assert_eq!(
        line_byte_lengths_threshold_crossing("a b c\n\r\nup\tdown", 0),
        Some((0, 5))
    );
    assert_eq!(
        line_byte_lengths_threshold_crossing("\n\nzz\n", 1),
        Some((2, 2))
    );
    assert_eq!(
        line_byte_lengths_threshold_crossing("nine\nq\nthree\nx\n", 6),
        None
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1246::TRACK_PROOF, 0x52540000 | 1246);
}
