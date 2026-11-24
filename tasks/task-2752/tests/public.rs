use task_2752::*;

#[test]
fn public_examples() {
    assert_eq!(parsed_values_total(""), 0);
    assert_eq!(parsed_values_total("a=1"), 1);
}
