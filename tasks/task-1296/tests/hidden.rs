use task_1296::*;

#[test]
fn edge_cases() {
    assert_eq!(
        command_arguments_threshold_crossing("only", 0),
        Some((0, 4))
    );
    assert_eq!(
        command_arguments_threshold_crossing("-q -q -- -v", 1),
        Some((0, 2))
    );
    assert_eq!(
        command_arguments_threshold_crossing("--level=3 data --level=9", 9),
        Some((0, 9))
    );
    assert_eq!(
        command_arguments_threshold_crossing("a --bad --level=x b", 256),
        None
    );
    assert_eq!(
        command_arguments_threshold_crossing("input -v -q input --level=255", 0),
        Some((0, 5))
    );
    assert_eq!(
        command_arguments_threshold_crossing("-- --level=4 x", 1),
        Some((0, 2))
    );
    assert_eq!(
        command_arguments_threshold_crossing("input -v -q input --level=255", 12),
        None
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1296::TRACK_PROOF, 0x52540000 | 1296);
}
