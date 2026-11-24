use task_444::*;

#[test]
fn public_examples() {
    assert_eq!(
        {
            let mut data = String::from("72;12");
            prefix_before_final_mark_equals(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("猫=12;note");
            prefix_before_final_mark_equals(&mut data);
            data
        },
        "猫"
    );
}
