use task_2703::*;

#[test]
fn edge_cases() {
    assert_eq!(
        cli_basenames_bounds("src/lib.rs readme docs/notes.md"),
        Some((6u32, 8u32))
    );
    assert_eq!(
        cli_basenames_bounds("one/../two ./three one//end/"),
        Some((3u32, 5u32))
    );
    assert_eq!(
        cli_basenames_bounds(".env report.tar.gz tmp"),
        Some((3u32, 13u32))
    );
    assert_eq!(
        cli_basenames_bounds("a a/z.rs a/b.rs .cargo/config"),
        Some((1u32, 6u32))
    );
    assert_eq!(
        cli_basenames_bounds("input.release output.log /tmp/ok"),
        Some((2u32, 13u32))
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2703::TRACK_PROOF, 0x52540000 | 2703);
}
