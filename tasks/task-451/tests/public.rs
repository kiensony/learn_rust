use task_451::*;

#[test]
fn public_examples() {
    assert_eq!(
        {
            let mut data = String::from("outside[red#blue]ignored");
            closing_marks_removed_hash(&mut data);
            data
        },
        "red#blue"
    );
    assert_eq!(
        {
            let mut data = String::from("[猫###a]");
            closing_marks_removed_hash(&mut data);
            data
        },
        "猫###a"
    );
}
