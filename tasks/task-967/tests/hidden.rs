use task_967::*;

#[test]
fn edge_cases() {
    assert_eq!(remainder_after_first_mark_slash(vec![]), vec![]);
    assert_eq!(
        remainder_after_first_mark_slash(vec![
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
            4usize, 0usize, 0usize, 0usize, 11usize, 2usize, 4usize, 4usize, 0usize, 0usize
        ]
    );
    assert_eq!(
        remainder_after_first_mark_slash(vec![
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
            2usize, 0usize, 3usize, 4usize, 0usize, 0usize, 0usize, 11usize, 2usize, 4usize,
            4usize, 0usize, 0usize, 2usize
        ]
    );
    assert_eq!(
        remainder_after_first_mark_slash(vec!["", "/", "/"]),
        vec![0usize, 0usize, 0usize]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_967::TRACK_PROOF, 0x52540000 | 967);
}
