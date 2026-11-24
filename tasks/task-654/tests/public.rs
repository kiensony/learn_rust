use task_654::*;

#[test]
fn public_examples() {
    assert_eq!(
        longest_field_with_last_tie_hash(&Record {
            label: String::from("outside[red#blue]ignored"),
            revision: 0
        }),
        4
    );
    assert_eq!(
        longest_field_with_last_tie_hash(&Record {
            label: String::from("unclosed[cat"),
            revision: 2
        }),
        2
    );
}
