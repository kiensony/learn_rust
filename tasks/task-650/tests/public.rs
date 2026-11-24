use task_650::*;

#[test]
fn public_examples() {
    assert_eq!(
        marks_removed_at_both_ends_hash(&Record {
            label: String::from("outside[red#blue]ignored"),
            revision: 0
        }),
        8
    );
    assert_eq!(
        marks_removed_at_both_ends_hash(&Record {
            label: String::from("unclosed[cat"),
            revision: 2
        }),
        2
    );
}
