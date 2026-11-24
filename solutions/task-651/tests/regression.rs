use task_651::*;

#[test]
fn transfer_case() {
    assert_eq!(
        remainder_after_first_mark_hash(&Record {
            label: String::from("ok#🐈"),
            revision: 13
        }),
        13
    );
}
