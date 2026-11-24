use task_760::*;

#[test]
fn edge_cases() {
    assert_eq!(
        first_nonempty_field_colon(&[]),
        std::collections::BTreeMap::from([])
    );
    assert_eq!(
        first_nonempty_field_colon(&["hi:kit", "hi:kit", ":é:a"]),
        std::collections::BTreeMap::from([
            (String::from("hi"), 2usize),
            (String::from("é"), 1usize)
        ])
    );
    assert_eq!(
        first_nonempty_field_colon(&["cat", "::one:猫:ab", "aa:bb", "ok:🐈"]),
        std::collections::BTreeMap::from([
            (String::from("cat"), 1usize),
            (String::from("one"), 1usize),
            (String::from("aa"), 1usize),
            (String::from("ok"), 1usize)
        ])
    );
    assert_eq!(
        first_nonempty_field_colon(&["", "", ":", ""]),
        std::collections::BTreeMap::from([(String::from(""), 4usize)])
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_760::TRACK_PROOF, 0x52540000 | 760);
}
