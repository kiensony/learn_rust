use task_2708::*;

#[test]
fn edge_cases() {
    assert_eq!(
        cli_basenames_steps("src/lib.rs readme docs/notes.md"),
        vec![0u32, 2u32]
    );
    assert_eq!(
        cli_basenames_steps("one/../two ./three one//end/"),
        vec![2u32, 2u32]
    );
    assert_eq!(
        cli_basenames_steps(".env report.tar.gz tmp"),
        vec![9u32, 10u32]
    );
    assert_eq!(
        cli_basenames_steps("a a/z.rs a/b.rs .cargo/config"),
        vec![3u32, 0u32, 2u32]
    );
    assert_eq!(
        cli_basenames_steps("input.release output.log /tmp/ok"),
        vec![3u32, 8u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2708::TRACK_PROOF, 0x52540000 | 2708);
}
