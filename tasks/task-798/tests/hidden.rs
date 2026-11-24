use task_798::*;

#[test]
fn edge_cases() {
    assert_eq!(
        remainder_after_first_mark_hash(&[]),
        std::collections::BTreeMap::from([])
    );
    assert_eq!(
        remainder_after_first_mark_hash(&[
            "outside[red#blue]ignored",
            "outside[red#blue]ignored",
            "[猫###a]"
        ]),
        std::collections::BTreeMap::from([
            (String::from("blue"), 2usize),
            (String::from("##a"), 1usize)
        ])
    );
    assert_eq!(
        remainder_after_first_mark_hash(&[
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
        std::collections::BTreeMap::from([(String::from(""), 10usize)])
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_798::TRACK_PROOF, 0x52540000 | 798);
}
