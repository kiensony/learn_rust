use task_800::*;

#[test]
fn transfer_case() {
    assert_eq!(
        first_nonempty_field_hash(&["", "", "#", ""]),
        std::collections::BTreeMap::from([(String::from(""), 4usize)])
    );
}
