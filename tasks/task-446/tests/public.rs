use task_446::*;

#[test]
fn public_examples() {
    assert_eq!(
        {
            let mut data = String::from("72;12");
            longest_field_with_last_tie_equals(&mut data);
            data
        },
        "72"
    );
    assert_eq!(
        {
            let mut data = String::from("猫=12;note");
            longest_field_with_last_tie_equals(&mut data);
            data
        },
        "猫"
    );
}
