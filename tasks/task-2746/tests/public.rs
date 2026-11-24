use task_2746::*;

#[test]
fn public_examples() {
    assert_eq!(
        cli_basenames_threshold_crossing("a a/z.rs a/b.rs .cargo/config", 0),
        Some((0, 1))
    );
    assert_eq!(cli_basenames_threshold_crossing("", 0), None);
}
