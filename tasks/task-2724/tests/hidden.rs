use task_2724::*;

#[test]
fn edge_cases() {
    assert_eq!(
        cli_path_depth_peaks("src/lib.rs readme docs/notes.md"),
        vec![2u32]
    );
    assert_eq!(
        cli_path_depth_peaks("one/../two ./three one//end/"),
        vec![2u32]
    );
    assert_eq!(cli_path_depth_peaks(".env report.tar.gz tmp"), vec![1u32]);
    assert_eq!(
        cli_path_depth_peaks("a a/z.rs a/b.rs .cargo/config"),
        vec![1u32, 2u32]
    );
    assert_eq!(
        cli_path_depth_peaks("input.release output.log /tmp/ok"),
        vec![1u32, 2u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2724::TRACK_PROOF, 0x52540000 | 2724);
}
