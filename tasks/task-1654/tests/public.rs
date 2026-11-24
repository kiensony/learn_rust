use task_1654::*;

#[test]
fn public_examples() {
    assert_eq!(major_version_peaks(""), vec![]);
    assert_eq!(major_version_peaks("0.0.1"), vec![0u32]);
}
