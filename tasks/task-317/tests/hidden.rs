use task_317::*;

#[test]
fn edge_cases() {
    assert_eq!(shortest_field_with_first_tie_colon(String::from("")), "");
    assert_eq!(shortest_field_with_first_tie_colon(String::from(":")), "");
    assert_eq!(
        shortest_field_with_first_tie_colon(String::from("cat")),
        "cat"
    );
    assert_eq!(
        shortest_field_with_first_tie_colon(String::from("::one:猫:ab")),
        ""
    );
    assert_eq!(
        shortest_field_with_first_tie_colon(String::from("aa:bb")),
        "aa"
    );
    assert_eq!(
        shortest_field_with_first_tie_colon(String::from("ok:🐈")),
        "ok"
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_317::TRACK_PROOF, 0x52540000 | 317);
}
