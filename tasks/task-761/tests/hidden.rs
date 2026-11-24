use task_761::*;

#[test]
fn edge_cases() {
    assert_eq!(
        longest_field_with_last_tie_colon(&[]),
        std::collections::BTreeMap::from([])
    );
    assert_eq!(
        longest_field_with_last_tie_colon(&["hi:kit", "hi:kit", ":é:a"]),
        std::collections::BTreeMap::from([
            (String::from("kit"), 2usize),
            (String::from("é"), 1usize)
        ])
    );
    assert_eq!(
        longest_field_with_last_tie_colon(&["cat", "::one:猫:ab", "aa:bb", "ok:🐈"]),
        std::collections::BTreeMap::from([
            (String::from("cat"), 1usize),
            (String::from("猫"), 1usize),
            (String::from("bb"), 1usize),
            (String::from("🐈"), 1usize)
        ])
    );
    assert_eq!(
        longest_field_with_last_tie_colon(&["", "", ":", ""]),
        std::collections::BTreeMap::from([(String::from(""), 4usize)])
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_761::TRACK_PROOF, 0x52540000 | 761);
}
