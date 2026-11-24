use task_452::*;

#[test]
fn public_examples() {
    assert_eq!(
        {
            let mut data = String::from("outside[red#blue]ignored");
            marks_removed_at_both_ends_hash(&mut data);
            data
        },
        "red#blue"
    );
    assert_eq!(
        {
            let mut data = String::from("[猫###a]");
            marks_removed_at_both_ends_hash(&mut data);
            data
        },
        "猫###a"
    );
}
