use task_442::*;

#[test]
fn public_examples() {
    assert_eq!(
        {
            let mut data = String::from("72;12");
            marks_removed_at_both_ends_equals(&mut data);
            data
        },
        "72"
    );
    assert_eq!(
        {
            let mut data = String::from("猫=12;note");
            marks_removed_at_both_ends_equals(&mut data);
            data
        },
        "猫=12"
    );
}
