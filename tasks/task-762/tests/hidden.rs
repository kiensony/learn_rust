use task_762::*;

#[test]
fn edge_cases() {
    assert_eq!(
        shortest_field_with_first_tie_colon(&[]),
        std::collections::BTreeMap::from([])
    );
    assert_eq!(
        shortest_field_with_first_tie_colon(&["hi:kit", "hi:kit", ":é:a"]),
        std::collections::BTreeMap::from([
            (String::from("hi"), 2usize),
            (String::from(""), 1usize)
        ])
    );
    assert_eq!(
        shortest_field_with_first_tie_colon(&["cat", "::one:猫:ab", "aa:bb", "ok:🐈"]),
        std::collections::BTreeMap::from([
            (String::from("cat"), 1usize),
            (String::from(""), 1usize),
            (String::from("aa"), 1usize),
            (String::from("ok"), 1usize)
        ])
    );
    assert_eq!(
        shortest_field_with_first_tie_colon(&["", "", ":", ""]),
        std::collections::BTreeMap::from([(String::from(""), 4usize)])
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_762::TRACK_PROOF, 0x52540000 | 762);
}
