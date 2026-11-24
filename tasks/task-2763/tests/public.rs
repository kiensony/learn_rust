use task_2763::*;

#[test]
fn public_examples() {
    assert_eq!(parsed_key_lengths_bounds(""), None);
    assert_eq!(parsed_key_lengths_bounds("a=1"), Some((1u32, 1u32)));
}
