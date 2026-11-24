use task_648::*;

#[test]
fn public_examples() {
    assert_eq!(
        leading_marks_removed_hash(&Record {
            label: String::from("outside[red#blue]ignored"),
            revision: 0
        }),
        8
    );
    assert_eq!(
        leading_marks_removed_hash(&Record {
            label: String::from("unclosed[cat"),
            revision: 2
        }),
        2
    );
}
