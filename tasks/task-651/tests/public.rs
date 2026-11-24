use task_651::*;

#[test]
fn public_examples() {
    assert_eq!(
        remainder_after_first_mark_hash(&Record {
            label: String::from("outside[red#blue]ignored"),
            revision: 0
        }),
        4
    );
    assert_eq!(
        remainder_after_first_mark_hash(&Record {
            label: String::from("unclosed[cat"),
            revision: 2
        }),
        2
    );
}
