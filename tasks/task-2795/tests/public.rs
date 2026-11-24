use task_2795::*;

#[test]
fn public_examples() {
    assert_eq!(final_settings_changes(""), vec![]);
    assert_eq!(final_settings_changes("a=1"), vec![1u32]);
}
