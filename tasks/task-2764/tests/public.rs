use task_2764::*;

#[test]
fn public_examples() {
    assert_eq!(parsed_key_lengths_peaks(""), vec![]);
    assert_eq!(parsed_key_lengths_peaks("a=1"), vec![1u32]);
}
