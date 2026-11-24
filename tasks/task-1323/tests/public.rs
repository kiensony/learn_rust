use task_1323::*;

#[test]
fn public_examples() {
    assert_eq!(flag_fields_bounds(""), None);
    assert_eq!(flag_fields_bounds("1"), Some((1u32, 1u32)));
}
