use task_445::*;

#[test]
fn public_examples() {
    assert_eq!(
        {
            let mut data = String::from("72;12");
            first_nonempty_field_equals(&mut data);
            data
        },
        "72"
    );
    assert_eq!(
        {
            let mut data = String::from("猫=12;note");
            first_nonempty_field_equals(&mut data);
            data
        },
        "猫"
    );
}
