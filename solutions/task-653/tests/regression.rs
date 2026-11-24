use task_653::*;

#[test]
fn transfer_case() {
    assert_eq!(
        first_nonempty_field_hash(&Record {
            label: String::from("ok#🐈"),
            revision: 13
        }),
        13
    );
}
