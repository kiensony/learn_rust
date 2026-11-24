use task_644::*;

#[test]
fn edge_cases() {
    assert_eq!(
        longest_field_with_last_tie_equals(&Record {
            label: String::from("hi=kit"),
            revision: 2
        }),
        5
    );
    assert_eq!(
        longest_field_with_last_tie_equals(&Record {
            label: String::from("=é=a"),
            revision: 3
        }),
        5
    );
    assert_eq!(
        longest_field_with_last_tie_equals(&Record {
            label: String::from(""),
            revision: 4
        }),
        4
    );
    assert_eq!(
        longest_field_with_last_tie_equals(&Record {
            label: String::from("="),
            revision: 5
        }),
        5
    );
    assert_eq!(
        longest_field_with_last_tie_equals(&Record {
            label: String::from("cat"),
            revision: 6
        }),
        9
    );
    assert_eq!(
        longest_field_with_last_tie_equals(&Record {
            label: String::from("==one=猫=ab"),
            revision: 7
        }),
        10
    );
    assert_eq!(
        longest_field_with_last_tie_equals(&Record {
            label: String::from("aa=bb"),
            revision: 8
        }),
        10
    );
    assert_eq!(
        longest_field_with_last_tie_equals(&Record {
            label: String::from("ok=🐈"),
            revision: 9
        }),
        13
    );
    assert_eq!(
        longest_field_with_last_tie_equals(&Record {
            label: String::from("aa=bb;meta=123"),
            revision: 10
        }),
        12
    );
    assert_eq!(
        longest_field_with_last_tie_equals(&Record {
            label: String::from(";ignore=all"),
            revision: 11
        }),
        11
    );
    assert_eq!(
        longest_field_with_last_tie_equals(&Record {
            label: String::from("a==b;"),
            revision: 12
        }),
        13
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_644::TRACK_PROOF, 0x52540000 | 644);
}
