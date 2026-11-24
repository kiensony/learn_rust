use task_2723::*;

#[test]
fn edge_cases() {
    assert_eq!(
        cli_path_depth_bounds("src/lib.rs readme docs/notes.md"),
        Some((1u32, 2u32))
    );
    assert_eq!(
        cli_path_depth_bounds("one/../two ./three one//end/"),
        Some((1u32, 2u32))
    );
    assert_eq!(
        cli_path_depth_bounds(".env report.tar.gz tmp"),
        Some((1u32, 1u32))
    );
    assert_eq!(
        cli_path_depth_bounds("a a/z.rs a/b.rs .cargo/config"),
        Some((1u32, 2u32))
    );
    assert_eq!(
        cli_path_depth_bounds("input.release output.log /tmp/ok"),
        Some((1u32, 2u32))
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2723::TRACK_PROOF, 0x52540000 | 2723);
}
