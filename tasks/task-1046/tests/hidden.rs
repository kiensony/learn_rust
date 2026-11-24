use task_1046::*;

#[test]
fn edge_cases() {
    assert_eq!(marks_removed_at_both_ends_hash(["unclosed[cat"]), [0]);
    assert_eq!(
        marks_removed_at_both_ends_hash([
            String::from("[]"),
            String::from("[42]"),
            String::from("[321]"),
            String::from("hi#kit"),
            String::from("#é#a"),
            String::from(""),
            String::from("#"),
            String::from("cat"),
            String::from("##one#猫#ab"),
            String::from("aa#bb"),
            String::from("ok#🐈")
        ]),
        [0, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0]
    );
    assert_eq!(
        marks_removed_at_both_ends_hash([
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
        [8, 7, 0, 0, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 8]
    );
    assert_eq!(
        marks_removed_at_both_ends_hash([String::from(""), String::from("#"), String::from("#")]),
        [0, 0, 0]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1046::TRACK_PROOF, 0x52540000 | 1046);
}
