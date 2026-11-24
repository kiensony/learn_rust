use task_437::*;

#[test]
fn public_examples() {
    assert_eq!(
        {
            let mut data = String::from("first\n123");
            shortest_field_with_first_tie_pipe(&mut data);
            data
        },
        "123"
    );
    assert_eq!(
        {
            let mut data = String::from("header\n猫|123");
            shortest_field_with_first_tie_pipe(&mut data);
            data
        },
        "猫"
    );
}
