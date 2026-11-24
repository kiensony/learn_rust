use task_2792::*;

#[test]
fn public_examples() {
    assert_eq!(final_settings_total(""), 0);
    assert_eq!(final_settings_total("a=1"), 1);
}
