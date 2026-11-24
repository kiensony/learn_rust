use task_1673::*;

#[test]
fn public_examples() {
    assert_eq!(patch_version_bounds(""), None);
    assert_eq!(patch_version_bounds("0.0.1"), Some((1u32, 1u32)));
}
