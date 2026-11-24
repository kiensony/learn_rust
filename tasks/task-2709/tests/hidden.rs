use task_2709::*;

#[test]
fn edge_cases() {
    assert_eq!(
        cli_basenames_longest_run("src/lib.rs readme docs/notes.md"),
        2
    );
    assert_eq!(cli_basenames_longest_run("one/../two ./three one//end/"), 1);
    assert_eq!(cli_basenames_longest_run(".env report.tar.gz tmp"), 1);
    assert_eq!(
        cli_basenames_longest_run("a a/z.rs a/b.rs .cargo/config"),
        2
    );
    assert_eq!(
        cli_basenames_longest_run("input.release output.log /tmp/ok"),
        1
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2709::TRACK_PROOF, 0x52540000 | 2709);
}
