use task_627::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_field_pipe(&Record {
            label: String::from("first\n123"),
            revision: 0
        }),
        3
    );
    assert_eq!(
        closing_field_pipe(&Record {
            label: String::from("header\n猫|123"),
            revision: 1
        }),
        4
    );
}
