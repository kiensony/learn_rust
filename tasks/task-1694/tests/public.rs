use task_1694::*;

#[test]
fn public_examples() {
    assert_eq!(release_line_peaks(""), vec![]);
    assert_eq!(release_line_peaks("0.0.1"), vec![0u32]);
}
