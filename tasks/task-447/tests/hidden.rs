use task_447::*;

#[test]
fn edge_cases() {
    assert_eq!(
        {
            let mut data = String::from("hi=kit");
            shortest_field_with_first_tie_equals(&mut data);
            data
        },
        "hi"
    );
    assert_eq!(
        {
            let mut data = String::from("=é=a");
            shortest_field_with_first_tie_equals(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("");
            shortest_field_with_first_tie_equals(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("=");
            shortest_field_with_first_tie_equals(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("cat");
            shortest_field_with_first_tie_equals(&mut data);
            data
        },
        "cat"
    );
    assert_eq!(
        {
            let mut data = String::from("==one=猫=ab");
            shortest_field_with_first_tie_equals(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("aa=bb");
            shortest_field_with_first_tie_equals(&mut data);
            data
        },
        "aa"
    );
    assert_eq!(
        {
            let mut data = String::from("ok=🐈");
            shortest_field_with_first_tie_equals(&mut data);
            data
        },
        "ok"
    );
    assert_eq!(
        {
            let mut data = String::from("aa=bb;meta=123");
            shortest_field_with_first_tie_equals(&mut data);
            data
        },
        "aa"
    );
    assert_eq!(
        {
            let mut data = String::from(";ignore=all");
            shortest_field_with_first_tie_equals(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("a==b;");
            shortest_field_with_first_tie_equals(&mut data);
            data
        },
        ""
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_447::TRACK_PROOF, 0x52540000 | 447);
}
