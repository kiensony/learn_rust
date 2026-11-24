use task_457::*;

#[test]
fn public_examples() {
    assert_eq!(
        {
            let mut data = String::from("outside[red#blue]ignored");
            shortest_field_with_first_tie_hash(&mut data);
            data
        },
        "red"
    );
    assert_eq!(
        {
            let mut data = String::from("[猫###a]");
            shortest_field_with_first_tie_hash(&mut data);
            data
        },
        ""
    );
}
