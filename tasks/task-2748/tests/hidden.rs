use task_2748::*;

#[test]
fn edge_cases() {
    assert_eq!(cli_path_depth_rolling_load("a.txt", 1), vec![1u64]);
    assert_eq!(
        cli_path_depth_rolling_load("src/lib.rs readme docs/notes.md", 2),
        vec![3u64, 3u64]
    );
    assert_eq!(
        cli_path_depth_rolling_load("one/../two ./three one//end/", 0),
        vec![]
    );
    assert_eq!(
        cli_path_depth_rolling_load(".env report.tar.gz tmp", 100),
        vec![]
    );
    assert_eq!(
        cli_path_depth_rolling_load("a a/z.rs a/b.rs .cargo/config", 3),
        vec![5u64, 6u64]
    );
    assert_eq!(
        cli_path_depth_rolling_load("input.release output.log /tmp/ok", 1),
        vec![1u64, 1u64, 2u64]
    );
    assert_eq!(
        cli_path_depth_rolling_load("a a/z.rs a/b.rs .cargo/config", 2),
        vec![3u64, 4u64, 4u64]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2748::TRACK_PROOF, 0x52540000 | 2748);
}
