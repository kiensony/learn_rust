use task_646::*;

#[test]
fn public_examples() {
    assert_eq!(
        leading_field_hash(&Record {
            label: String::from("outside[red#blue]ignored"),
            revision: 0
        }),
        3
    );
    assert_eq!(
        leading_field_hash(&Record {
            label: String::from("[猫###a]"),
            revision: 1
        }),
        4
    );
}
