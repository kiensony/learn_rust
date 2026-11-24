use task_647::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_field_hash(&Record {
            label: String::from("outside[red#blue]ignored"),
            revision: 0
        }),
        4
    );
    assert_eq!(
        closing_field_hash(&Record {
            label: String::from("[猫###a]"),
            revision: 1
        }),
        2
    );
}
