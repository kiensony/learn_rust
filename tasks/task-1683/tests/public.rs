use task_1683::*;

#[test]
fn public_examples() {
    assert_eq!(preview_version_bounds(""), None);
    assert_eq!(preview_version_bounds("0.0.1"), Some((1u32, 1u32)));
}
