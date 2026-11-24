use task_367::*;

#[test]
fn edge_cases() {
    assert_eq!(shortest_field_with_first_tie_colon(&String::from("")), 0);
    assert_eq!(shortest_field_with_first_tie_colon(&String::from(":")), 0);
    assert_eq!(shortest_field_with_first_tie_colon(&String::from("cat")), 3);
    assert_eq!(
        shortest_field_with_first_tie_colon(&String::from("::one:猫:ab")),
        0
    );
    assert_eq!(
        shortest_field_with_first_tie_colon(&String::from("aa:bb")),
        2
    );
    assert_eq!(
        shortest_field_with_first_tie_colon(&String::from("ok:🐈")),
        2
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_367::TRACK_PROOF, 0x52540000 | 367);
}
