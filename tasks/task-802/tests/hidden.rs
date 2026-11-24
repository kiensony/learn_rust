use task_802::*;

#[test]
fn edge_cases() {
    assert_eq!(
        shortest_field_with_first_tie_hash(&[]),
        std::collections::BTreeMap::from([])
    );
    assert_eq!(
        shortest_field_with_first_tie_hash(&[
            "outside[red#blue]ignored",
            "outside[red#blue]ignored",
            "[猫###a]"
        ]),
        std::collections::BTreeMap::from([
            (String::from("red"), 2usize),
            (String::from(""), 1usize)
        ])
    );
    assert_eq!(
        shortest_field_with_first_tie_hash(&[
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
}

#[test]
fn submission_marker() {
    assert_eq!(task_802::TRACK_PROOF, 0x52540000 | 802);
}
