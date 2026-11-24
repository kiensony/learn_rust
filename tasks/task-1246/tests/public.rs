use task_1246::*;

#[test]
fn public_examples() {
    assert_eq!(
        line_byte_lengths_threshold_crossing("nine\nq\nthree\nx\n", 0),
        Some((0, 4))
    );
    assert_eq!(line_byte_lengths_threshold_crossing("", 0), None);
}
