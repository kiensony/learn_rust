use task_755::*;

#[test]
fn edge_cases() {
    assert_eq!(
        closing_field_colon(&[]),
        std::collections::BTreeMap::from([])
    );
    assert_eq!(
        closing_field_colon(&["hi:kit", "hi:kit", ":é:a"]),
        std::collections::BTreeMap::from([
            (String::from("kit"), 2usize),
            (String::from("a"), 1usize)
        ])
    );
    assert_eq!(
        closing_field_colon(&["cat", "::one:猫:ab", "aa:bb", "ok:🐈"]),
        std::collections::BTreeMap::from([
            (String::from("cat"), 1usize),
            (String::from("ab"), 1usize),
            (String::from("bb"), 1usize),
            (String::from("🐈"), 1usize)
        ])
    );
    assert_eq!(
        closing_field_colon(&["", "", ":", ""]),
        std::collections::BTreeMap::from([(String::from(""), 4usize)])
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_755::TRACK_PROOF, 0x52540000 | 755);
}
