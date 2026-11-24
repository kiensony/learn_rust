use task_449::*;

#[test]
fn public_examples() {
    assert_eq!(
        {
            let mut data = String::from("outside[red#blue]ignored");
            closing_field_hash(&mut data);
            data
        },
        "blue"
    );
    assert_eq!(
        {
            let mut data = String::from("[猫###a]");
            closing_field_hash(&mut data);
            data
        },
        "a"
    );
}
