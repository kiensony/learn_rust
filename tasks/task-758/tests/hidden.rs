use task_758::*;

#[test]
fn edge_cases() {
    assert_eq!(
        marks_removed_at_both_ends_colon(&[]),
        std::collections::BTreeMap::from([])
    );
    assert_eq!(
        marks_removed_at_both_ends_colon(&["hi:kit", "hi:kit", ":é:a"]),
        std::collections::BTreeMap::from([
            (String::from("hi:kit"), 2usize),
            (String::from("é:a"), 1usize)
        ])
    );
    assert_eq!(
        marks_removed_at_both_ends_colon(&["cat", "::one:猫:ab", "aa:bb", "ok:🐈"]),
        std::collections::BTreeMap::from([
            (String::from("cat"), 1usize),
            (String::from("one:猫:ab"), 1usize),
            (String::from("aa:bb"), 1usize),
            (String::from("ok:🐈"), 1usize)
        ])
    );
    assert_eq!(
        marks_removed_at_both_ends_colon(&["", "", ":", ""]),
        std::collections::BTreeMap::from([(String::from(""), 4usize)])
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_758::TRACK_PROOF, 0x52540000 | 758);
}
