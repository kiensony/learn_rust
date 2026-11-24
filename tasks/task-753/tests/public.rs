use task_753::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_hash(Some("outside[red#blue]ignored")),
        None
    );
    assert_eq!(shortest_field_with_first_tie_hash(Some("[42]")), Some(42));
}
