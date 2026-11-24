use task_432::*;

#[test]
fn public_examples() {
    assert_eq!(
        {
            let mut data = String::from("first\n123");
            marks_removed_at_both_ends_pipe(&mut data);
            data
        },
        "123"
    );
    assert_eq!(
        {
            let mut data = String::from("header\n猫|123");
            marks_removed_at_both_ends_pipe(&mut data);
            data
        },
        "猫|123"
    );
}
