use task_2793::*;

#[test]
fn public_examples() {
    assert_eq!(final_settings_bounds(""), None);
    assert_eq!(final_settings_bounds("a=1"), Some((1u32, 1u32)));
}
