use task_456::*;

#[test]
fn public_examples() {
    assert_eq!(
        {
            let mut data = String::from("outside[red#blue]ignored");
            longest_field_with_last_tie_hash(&mut data);
            data
        },
        "blue"
    );
    assert_eq!(
        {
            let mut data = String::from("[猫###a]");
            longest_field_with_last_tie_hash(&mut data);
            data
        },
        "猫"
    );
}
