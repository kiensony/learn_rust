use task_1491::*;

#[test]
fn public_examples() {
    assert_eq!(status_detail_trace(""), vec![]);
    assert_eq!(status_detail_trace("200"), vec![0u32]);
}
