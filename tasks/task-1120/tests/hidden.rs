use task_1120::*;

#[test]
fn edge_cases() {
    assert_eq!(shortest_field_with_first_tie_slash(&"hi/kit"), 2);
    assert_eq!(
        shortest_field_with_first_tie_slash(&String::from("/é/a")),
        0
    );
    assert_eq!(shortest_field_with_first_tie_slash(&""), 0);
    assert_eq!(shortest_field_with_first_tie_slash(&String::from("/")), 0);
    assert_eq!(shortest_field_with_first_tie_slash(&"cat"), 3);
    assert_eq!(
        shortest_field_with_first_tie_slash(&String::from("//one/猫/ab")),
        0
    );
    assert_eq!(shortest_field_with_first_tie_slash(&"aa/bb"), 2);
    assert_eq!(
        shortest_field_with_first_tie_slash(&String::from("ok/🐈")),
        2
    );
    assert_eq!(shortest_field_with_first_tie_slash(&"path/first/last"), 4);
    assert_eq!(
        shortest_field_with_first_tie_slash(&String::from("path/path/")),
        0
    );
    assert_eq!(shortest_field_with_first_tie_slash(&"path/"), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1120::TRACK_PROOF, 0x52540000 | 1120);
}
