use task_801::*;

#[test]
fn transfer_case() {
    assert_eq!(
        longest_field_with_last_tie_hash(&["", "", "#", ""]),
        std::collections::BTreeMap::from([(String::from(""), 4usize)])
    );
}
