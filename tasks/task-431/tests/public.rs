use task_431::*;

#[test]
fn public_examples() {
    assert_eq!(
        {
            let mut data = String::from("first\n123");
            closing_marks_removed_pipe(&mut data);
            data
        },
        "123"
    );
    assert_eq!(
        {
            let mut data = String::from("header\n猫|123");
            closing_marks_removed_pipe(&mut data);
            data
        },
        "猫|123"
    );
}
