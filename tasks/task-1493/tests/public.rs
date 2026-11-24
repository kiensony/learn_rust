use task_1493::*;

#[test]
fn public_examples() {
    assert_eq!(status_detail_bounds(""), None);
    assert_eq!(status_detail_bounds("200"), Some((0u32, 0u32)));
}
