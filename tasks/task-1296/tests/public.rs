use task_1296::*;

#[test]
fn public_examples() {
    assert_eq!(
        command_arguments_threshold_crossing("input -v -q input --level=255", 0),
        Some((0, 5))
    );
    assert_eq!(command_arguments_threshold_crossing("", 0), None);
}
