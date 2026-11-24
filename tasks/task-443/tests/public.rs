use task_443::*;

#[test]
fn public_examples() {
    assert_eq!(
        {
            let mut data = String::from("72;12");
            remainder_after_first_mark_equals(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("猫=12;note");
            remainder_after_first_mark_equals(&mut data);
            data
        },
        "12"
    );
}
