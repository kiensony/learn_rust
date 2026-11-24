use task_417::*;

#[test]
fn public_examples() {
    assert_eq!(
        {
            let mut data = String::from("hi:kit");
            shortest_field_with_first_tie_colon(&mut data);
            data
        },
        "hi"
    );
    assert_eq!(
        {
            let mut data = String::from(":é:a");
            shortest_field_with_first_tie_colon(&mut data);
            data
        },
        ""
    );
}
