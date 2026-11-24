use task_455::*;

#[test]
fn public_examples() {
    assert_eq!(
        {
            let mut data = String::from("outside[red#blue]ignored");
            first_nonempty_field_hash(&mut data);
            data
        },
        "red"
    );
    assert_eq!(
        {
            let mut data = String::from("[猫###a]");
            first_nonempty_field_hash(&mut data);
            data
        },
        "猫"
    );
}
