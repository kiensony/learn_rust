use task_1653::*;

#[test]
fn public_examples() {
    assert_eq!(major_version_bounds(""), None);
    assert_eq!(major_version_bounds("0.0.1"), Some((0u32, 0u32)));
}
