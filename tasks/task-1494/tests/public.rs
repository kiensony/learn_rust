use task_1494::*;

#[test]
fn public_examples() {
    assert_eq!(status_detail_peaks(""), vec![]);
    assert_eq!(status_detail_peaks("200"), vec![0u32]);
}
