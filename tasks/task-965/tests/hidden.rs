use task_965::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_marks_removed_slash(vec![]), vec![]);
    assert_eq!(
        closing_marks_removed_slash(vec![
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
            5usize, 0usize, 0usize, 3usize, 12usize, 5usize, 7usize, 10usize, 4usize, 0usize
        ]
    );
    assert_eq!(
        closing_marks_removed_slash(vec![
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
            4usize, 3usize, 6usize, 5usize, 0usize, 0usize, 3usize, 12usize, 5usize, 7usize,
            10usize, 4usize, 0usize, 4usize
        ]
    );
    assert_eq!(
        closing_marks_removed_slash(vec!["", "/", "/"]),
        vec![0usize, 0usize, 0usize]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_965::TRACK_PROOF, 0x52540000 | 965);
}
