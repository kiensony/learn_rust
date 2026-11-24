use task_631::*;

#[test]
fn public_examples() {
    assert_eq!(
        remainder_after_first_mark_pipe(&Record {
            label: String::from("first\n123"),
            revision: 0
        }),
        0
    );
    assert_eq!(
        remainder_after_first_mark_pipe(&Record {
            label: String::from("header\n猫|123"),
            revision: 1
        }),
        4
    );
}
