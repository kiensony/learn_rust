use task_1313::*;

#[test]
fn public_examples() {
    assert_eq!(nibble_fields_bounds(""), None);
    assert_eq!(nibble_fields_bounds("1"), Some((0u32, 0u32)));
}
