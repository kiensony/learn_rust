use task_966::*;

#[test]
fn edge_cases() {
    assert_eq!(marks_removed_at_both_ends_slash(vec![]), vec![]);
    assert_eq!(
        marks_removed_at_both_ends_slash(vec![
            "/é/a",
            "",
            "/",
            "cat",
            "//one/猫/ab",
            "aa/bb",
            "ok/🐈",
            "path/first/last",
            "path/path/",
            "path/"
        ]),
        vec![
            4usize, 0usize, 0usize, 3usize, 10usize, 5usize, 7usize, 10usize, 4usize, 0usize
        ]
    );
    assert_eq!(
        marks_removed_at_both_ends_slash(vec![
            "path/1/12",
            "path/123",
            "hi/kit",
            "/é/a",
            "",
            "/",
            "cat",
            "//one/猫/ab",
            "aa/bb",
            "ok/🐈",
            "path/first/last",
            "path/path/",
            "path/",
            "path/1/12"
        ]),
        vec![
            4usize, 3usize, 6usize, 4usize, 0usize, 0usize, 3usize, 10usize, 5usize, 7usize,
            10usize, 4usize, 0usize, 4usize
        ]
    );
    assert_eq!(
        marks_removed_at_both_ends_slash(vec!["", "/", "/"]),
        vec![0usize, 0usize, 0usize]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_966::TRACK_PROOF, 0x52540000 | 966);
}
