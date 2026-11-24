use task_797::*;

#[test]
fn edge_cases() {
    assert_eq!(
        marks_removed_at_both_ends_hash(&[]),
        std::collections::BTreeMap::from([])
    );
    assert_eq!(
        marks_removed_at_both_ends_hash(&[
            "outside[red#blue]ignored",
            "outside[red#blue]ignored",
            "[猫###a]"
        ]),
        std::collections::BTreeMap::from([
            (String::from("red#blue"), 2usize),
            (String::from("猫###a"), 1usize)
        ])
    );
    assert_eq!(
        marks_removed_at_both_ends_hash(&[
            "[42]",
            "[321]",
            "hi#kit",
            "#é#a",
            "",
            "#",
            "cat",
            "##one#猫#ab",
            "aa#bb",
            "ok#🐈"
        ]),
        std::collections::BTreeMap::from([
            (String::from("42"), 1usize),
            (String::from("321"), 1usize),
            (String::from(""), 8usize)
        ])
    );
    assert_eq!(
        marks_removed_at_both_ends_hash(&["", "", "#", ""]),
        std::collections::BTreeMap::from([(String::from(""), 4usize)])
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_797::TRACK_PROOF, 0x52540000 | 797);
}
