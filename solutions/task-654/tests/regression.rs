use task_654::*;

#[test]
fn transfer_case() {
    assert_eq!(
        longest_field_with_last_tie_hash(&Record {
            label: String::from("ok#🐈"),
            revision: 13
        }),
        13
    );
}
