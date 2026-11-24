use task_1011::*;

#[test]
fn edge_cases() {
    assert_eq!(shortest_field_with_first_tie_colon([""]), [0]);
    assert_eq!(
        shortest_field_with_first_tie_colon([
            String::from(":"),
            String::from("cat"),
            String::from("::one:猫:ab"),
            String::from("aa:bb"),
            String::from("ok:🐈")
        ]),
        [0, 3, 0, 2, 2]
    );
    assert_eq!(
        shortest_field_with_first_tie_colon([
            "hi:kit",
            ":é:a",
            "",
            ":",
            "cat",
            "::one:猫:ab",
            "aa:bb",
            "ok:🐈",
            "hi:kit"
        ]),
        [2, 0, 0, 0, 3, 0, 2, 2, 2]
    );
    assert_eq!(
        shortest_field_with_first_tie_colon([
            String::from(""),
            String::from(":"),
            String::from(":")
        ]),
        [0, 0, 0]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1011::TRACK_PROOF, 0x52540000 | 1011);
}
