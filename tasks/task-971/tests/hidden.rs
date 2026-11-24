use task_971::*;

#[test]
fn edge_cases() {
    assert_eq!(shortest_field_with_first_tie_slash(vec![]), vec![]);
    assert_eq!(
        shortest_field_with_first_tie_slash(vec![
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
            0usize, 0usize, 0usize, 3usize, 0usize, 2usize, 2usize, 4usize, 0usize, 0usize
        ]
    );
    assert_eq!(
        shortest_field_with_first_tie_slash(vec![
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
            1usize, 3usize, 2usize, 0usize, 0usize, 0usize, 3usize, 0usize, 2usize, 2usize, 4usize,
            0usize, 0usize, 1usize
        ]
    );
    assert_eq!(
        shortest_field_with_first_tie_slash(vec!["", "/", "/"]),
        vec![0usize, 0usize, 0usize]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_971::TRACK_PROOF, 0x52540000 | 971);
}
