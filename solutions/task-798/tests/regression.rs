use task_798::*;

#[test]
fn transfer_case() {
    assert_eq!(
        remainder_after_first_mark_hash(&["", "", "#", ""]),
        std::collections::BTreeMap::from([(String::from(""), 4usize)])
    );
}
