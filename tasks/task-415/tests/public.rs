use task_415::*;

#[test]
fn public_examples() {
    assert_eq!(
        {
            let mut data = String::from("hi:kit");
            first_nonempty_field_colon(&mut data);
            data
        },
        "hi"
    );
    assert_eq!(
        {
            let mut data = String::from(":é:a");
            first_nonempty_field_colon(&mut data);
            data
        },
        "é"
    );
}
