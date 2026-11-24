use task_447::*;

#[test]
fn public_examples() {
    assert_eq!(
        {
            let mut data = String::from("72;12");
            shortest_field_with_first_tie_equals(&mut data);
            data
        },
        "72"
    );
    assert_eq!(
        {
            let mut data = String::from("猫=12;note");
            shortest_field_with_first_tie_equals(&mut data);
            data
        },
        "12"
    );
}
