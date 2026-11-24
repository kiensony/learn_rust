use task_1001::*;

#[test]
fn transfer_case() {
    assert_eq!(
        shortest_field_with_first_tie_hash(vec!["", "#", "#"]),
        vec![0usize, 0usize, 0usize]
    );
}
