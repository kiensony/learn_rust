use task_2753::*;

#[test]
fn public_examples() {
    assert_eq!(parsed_values_bounds(""), None);
    assert_eq!(parsed_values_bounds("a=1"), Some((1u32, 1u32)));
}
