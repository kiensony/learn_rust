use task_653::*;

#[test]
fn public_examples() {
    assert_eq!(
        first_nonempty_field_hash(&Record {
            label: String::from("outside[red#blue]ignored"),
            revision: 0
        }),
        3
    );
    assert_eq!(
        first_nonempty_field_hash(&Record {
            label: String::from("[猫###a]"),
            revision: 1
        }),
        4
    );
}
