use task_630::*;

#[test]
fn public_examples() {
    assert_eq!(
        marks_removed_at_both_ends_pipe(&Record {
            label: String::from("first\n123"),
            revision: 0
        }),
        3
    );
    assert_eq!(
        marks_removed_at_both_ends_pipe(&Record {
            label: String::from("header\n猫|123"),
            revision: 1
        }),
        8
    );
}
