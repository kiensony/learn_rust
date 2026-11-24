use task_1043::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_field_hash(["unclosed[cat"]), [0]);
    assert_eq!(
        closing_field_hash([
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
        closing_field_hash([
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
        [4, 1, 0, 0, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 4]
    );
    assert_eq!(
        closing_field_hash([String::from(""), String::from("#"), String::from("#")]),
        [0, 0, 0]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1043::TRACK_PROOF, 0x52540000 | 1043);
}
