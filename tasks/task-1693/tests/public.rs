use task_1693::*;

#[test]
fn public_examples() {
    assert_eq!(release_line_bounds(""), None);
    assert_eq!(release_line_bounds("0.0.1"), Some((0u32, 0u32)));
}
