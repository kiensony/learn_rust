use task_1045::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_marks_removed_hash(["unclosed[cat"]), [0]);
    assert_eq!(
        closing_marks_removed_hash([
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
        closing_marks_removed_hash([
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
        closing_marks_removed_hash([String::from(""), String::from("#"), String::from("#")]),
        [0, 0, 0]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1045::TRACK_PROOF, 0x52540000 | 1045);
}
