use task_448::*;

#[test]
fn public_examples() {
    assert_eq!(
        {
            let mut data = String::from("outside[red#blue]ignored");
            leading_field_hash(&mut data);
            data
        },
        "red"
    );
    assert_eq!(
        {
            let mut data = String::from("[猫###a]");
            leading_field_hash(&mut data);
            data
        },
        "猫"
    );
}
