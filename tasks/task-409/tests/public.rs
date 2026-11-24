use task_409::*;

#[test]
fn public_examples() {
    assert_eq!(
        {
            let mut data = String::from("hi:kit");
            closing_field_colon(&mut data);
            data
        },
        "kit"
    );
    assert_eq!(
        {
            let mut data = String::from(":é:a");
            closing_field_colon(&mut data);
            data
        },
        "a"
    );
}
