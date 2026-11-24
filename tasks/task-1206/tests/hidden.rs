use task_1206::*;

#[test]
fn edge_cases() {
    assert_eq!(
        line_byte_lengths_descending_steps("a\nblue\nblue\r\nz\n"),
        1
    );
    assert_eq!(line_byte_lengths_descending_steps("   x\té\n\ncat\n"), 1);
    assert_eq!(line_byte_lengths_descending_steps("nine\nq\nthree\nx\n"), 2);
    assert_eq!(line_byte_lengths_descending_steps("a b c\n\r\nup\tdown"), 1);
    assert_eq!(line_byte_lengths_descending_steps("\n\nzz\n"), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1206::TRACK_PROOF, 0x52540000 | 1206);
}
