use task_968::*;

#[test]
fn edge_cases() {
    assert_eq!(prefix_before_final_mark_slash(vec![]), vec![]);
    assert_eq!(
        prefix_before_final_mark_slash(vec![
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
            3usize, 0usize, 0usize, 0usize, 9usize, 2usize, 2usize, 5usize, 4usize, 0usize
        ]
    );
    assert_eq!(
        prefix_before_final_mark_slash(vec![
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
            1usize, 0usize, 2usize, 3usize, 0usize, 0usize, 0usize, 9usize, 2usize, 2usize, 5usize,
            4usize, 0usize, 1usize
        ]
    );
    assert_eq!(
        prefix_before_final_mark_slash(vec!["", "/", "/"]),
        vec![0usize, 0usize, 0usize]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_968::TRACK_PROOF, 0x52540000 | 968);
}
