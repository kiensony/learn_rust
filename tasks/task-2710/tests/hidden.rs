use task_2710::*;

#[test]
fn edge_cases() {
    assert_eq!(
        cli_basenames_reverse("src/lib.rs readme docs/notes.md"),
        vec![8u32, 6u32, 6u32]
    );
    assert_eq!(
        cli_basenames_reverse("one/../two ./three one//end/"),
        vec![3u32, 5u32, 3u32]
    );
    assert_eq!(
        cli_basenames_reverse(".env report.tar.gz tmp"),
        vec![3u32, 13u32, 4u32]
    );
    assert_eq!(
        cli_basenames_reverse("a a/z.rs a/b.rs .cargo/config"),
        vec![6u32, 4u32, 4u32, 1u32]
    );
    assert_eq!(
        cli_basenames_reverse("input.release output.log /tmp/ok"),
        vec![2u32, 10u32, 13u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2710::TRACK_PROOF, 0x52540000 | 2710);
}
