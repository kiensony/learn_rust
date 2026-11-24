use task_645::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_equals(&Record {
            label: String::from("72;12"),
            revision: 0
        }),
        2
    );
    assert_eq!(
        shortest_field_with_first_tie_equals(&Record {
            label: String::from("猫=12;note"),
            revision: 1
        }),
        3
    );
}
