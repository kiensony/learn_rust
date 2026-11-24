use task_1495::*;

#[test]
fn public_examples() {
    assert_eq!(status_detail_changes(""), vec![]);
    assert_eq!(status_detail_changes("200"), vec![0u32]);
}
