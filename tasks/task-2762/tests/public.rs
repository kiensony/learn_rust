use task_2762::*;

#[test]
fn public_examples() {
    assert_eq!(parsed_key_lengths_total(""), 0);
    assert_eq!(parsed_key_lengths_total("a=1"), 1);
}
