use task_453::*;

#[test]
fn public_examples() {
    assert_eq!(
        {
            let mut data = String::from("outside[red#blue]ignored");
            remainder_after_first_mark_hash(&mut data);
            data
        },
        "blue"
    );
    assert_eq!(
        {
            let mut data = String::from("[猫###a]");
            remainder_after_first_mark_hash(&mut data);
            data
        },
        "##a"
    );
}
