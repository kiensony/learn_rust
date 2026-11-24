use task_397::*;

#[test]
fn edge_cases() {
    assert_eq!(
        shortest_field_with_first_tie_equals(&String::from("猫=12;note")),
        2
    );
    assert_eq!(
        shortest_field_with_first_tie_equals(&String::from("hi=kit")),
        2
    );
    assert_eq!(shortest_field_with_first_tie_equals(&String::from("")), 0);
    assert_eq!(shortest_field_with_first_tie_equals(&String::from("=")), 0);
    assert_eq!(
        shortest_field_with_first_tie_equals(&String::from("cat")),
        3
    );
    assert_eq!(
        shortest_field_with_first_tie_equals(&String::from("==one=猫=ab")),
        0
    );
    assert_eq!(
        shortest_field_with_first_tie_equals(&String::from("aa=bb")),
        2
    );
    assert_eq!(
        shortest_field_with_first_tie_equals(&String::from("ok=🐈")),
        2
    );
    assert_eq!(
        shortest_field_with_first_tie_equals(&String::from("aa=bb;meta=123")),
        2
    );
    assert_eq!(
        shortest_field_with_first_tie_equals(&String::from(";ignore=all")),
        0
    );
    assert_eq!(
        shortest_field_with_first_tie_equals(&String::from("a==b;")),
        0
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_397::TRACK_PROOF, 0x52540000 | 397);
}
