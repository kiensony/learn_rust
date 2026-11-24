use task_1696::*;

#[test]
fn transfer_case() {
    assert_eq!(
        major_version_threshold_crossing("1.0.0 1.2.0 2.0.0 1.0.0 1.2.0 2.0.0", 1),
        Some((0, 1))
    );
}
