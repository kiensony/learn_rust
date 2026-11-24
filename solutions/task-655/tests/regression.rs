use task_655::*;

#[test]
fn transfer_case() {
    assert_eq!(
        shortest_field_with_first_tie_hash(&Record {
            label: String::from("ok#🐈"),
            revision: 13
        }),
        13
    );
}
