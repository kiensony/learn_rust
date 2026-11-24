use task_435::*;

#[test]
fn public_examples() {
    assert_eq!(
        {
            let mut data = String::from("first\n123");
            first_nonempty_field_pipe(&mut data);
            data
        },
        "123"
    );
    assert_eq!(
        {
            let mut data = String::from("header\n猫|123");
            first_nonempty_field_pipe(&mut data);
            data
        },
        "猫"
    );
}
