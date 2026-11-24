use task_655::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_hash(&Record {
            label: String::from("outside[red#blue]ignored"),
            revision: 0
        }),
        3
    );
    assert_eq!(
        shortest_field_with_first_tie_hash(&Record {
            label: String::from("[猫###a]"),
            revision: 1
        }),
        1
    );
}
