use task_1691::*;

#[test]
fn public_examples() {
    assert_eq!(release_line_trace(""), vec![]);
    assert_eq!(release_line_trace("0.0.1"), vec![0u32]);
}
