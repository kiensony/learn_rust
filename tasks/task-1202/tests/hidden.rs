use task_1202::*;

#[test]
fn edge_cases() {
    assert_eq!(line_byte_lengths_total("a\nblue\nblue\r\nz\n"), 10);
    assert_eq!(line_byte_lengths_total("   x\té\n\ncat\n"), 10);
    assert_eq!(line_byte_lengths_total("nine\nq\nthree\nx\n"), 11);
    assert_eq!(line_byte_lengths_total("a b c\n\r\nup\tdown"), 12);
    assert_eq!(line_byte_lengths_total("\n\nzz\n"), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1202::TRACK_PROOF, 0x52540000 | 1202);
}
