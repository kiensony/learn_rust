use task_759::*;

#[test]
fn edge_cases() {
    assert_eq!(
        prefix_before_final_mark_colon(&[]),
        std::collections::BTreeMap::from([])
    );
    assert_eq!(
        prefix_before_final_mark_colon(&["hi:kit", "hi:kit", ":é:a"]),
        std::collections::BTreeMap::from([
            (String::from("hi"), 2usize),
            (String::from(":é"), 1usize)
        ])
    );
    assert_eq!(
        prefix_before_final_mark_colon(&["cat", "::one:猫:ab", "aa:bb", "ok:🐈"]),
        std::collections::BTreeMap::from([
            (String::from(""), 1usize),
            (String::from("::one:猫"), 1usize),
            (String::from("aa"), 1usize),
            (String::from("ok"), 1usize)
        ])
    );
    assert_eq!(
        prefix_before_final_mark_colon(&["", "", ":", ""]),
        std::collections::BTreeMap::from([(String::from(""), 4usize)])
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_759::TRACK_PROOF, 0x52540000 | 759);
}
