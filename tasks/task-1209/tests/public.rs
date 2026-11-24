use task_1209::*;

#[test]
fn public_examples() {
    assert_eq!(line_byte_lengths_longest_run(""), 0);
    assert_eq!(line_byte_lengths_longest_run("a"), 1);
}
