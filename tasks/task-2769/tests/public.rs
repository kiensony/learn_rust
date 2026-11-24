use task_2769::*;

#[test]
fn public_examples() {
    assert_eq!(parsed_key_lengths_longest_run(""), 0);
    assert_eq!(parsed_key_lengths_longest_run("a=1"), 1);
}
