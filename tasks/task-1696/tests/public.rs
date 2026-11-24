use task_1696::*;

#[test]
fn public_examples() {
    assert_eq!(
        major_version_threshold_crossing("1.0.0 1.2.0 2.0.0", 0),
        Some((0, 1))
    );
    assert_eq!(major_version_threshold_crossing("", 0), None);
}
