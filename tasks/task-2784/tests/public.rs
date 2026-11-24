use task_2784::*;

#[test]
fn public_examples() {
    assert_eq!(parsed_defaults_peaks(""), vec![]);
    assert_eq!(parsed_defaults_peaks("a=1"), vec![]);
}
