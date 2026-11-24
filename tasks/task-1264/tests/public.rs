use task_1264::*;

#[test]
fn public_examples() {
    assert_eq!(positional_options_peaks(""), vec![]);
    assert_eq!(positional_options_peaks("only"), vec![4u32]);
}
