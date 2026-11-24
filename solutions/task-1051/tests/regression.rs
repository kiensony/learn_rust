use task_1051::*;

#[test]
fn transfer_case() {
    assert_eq!(
        shortest_field_with_first_tie_hash([
            String::from(""),
            String::from("#"),
            String::from("#")
        ]),
        [0, 0, 0]
    );
}
