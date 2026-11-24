use task_408::*;

#[test]
fn public_examples() {
    assert_eq!(
        {
            let mut data = String::from("hi:kit");
            leading_field_colon(&mut data);
            data
        },
        "hi"
    );
    assert_eq!(
        {
            let mut data = String::from(":é:a");
            leading_field_colon(&mut data);
            data
        },
        ""
    );
}
