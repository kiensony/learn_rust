use task_993::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_field_hash(vec![]), vec![]);
    assert_eq!(
        closing_field_hash(vec![
            "[]",
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
        vec![
            0usize, 2usize, 3usize, 0usize, 0usize, 0usize, 0usize, 0usize, 0usize, 0usize, 0usize
        ]
    );
    assert_eq!(
        closing_field_hash(vec![
            "outside[red#blue]ignored",
            "[猫###a]",
            "unclosed[cat",
            "[]",
            "[42]",
            "[321]",
            "hi#kit",
            "#é#a",
            "",
            "#",
            "cat",
            "##one#猫#ab",
            "aa#bb",
            "ok#🐈",
            "outside[red#blue]ignored"
        ]),
        vec![
            4usize, 1usize, 0usize, 0usize, 2usize, 3usize, 0usize, 0usize, 0usize, 0usize, 0usize,
            0usize, 0usize, 0usize, 4usize
        ]
    );
    assert_eq!(
        closing_field_hash(vec!["", "#", "#"]),
        vec![0usize, 0usize, 0usize]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_993::TRACK_PROOF, 0x52540000 | 993);
}
