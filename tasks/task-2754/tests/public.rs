use task_2754::*;

#[test]
fn public_examples() {
    assert_eq!(parsed_values_peaks(""), vec![]);
    assert_eq!(parsed_values_peaks("a=1"), vec![1u32]);
}
