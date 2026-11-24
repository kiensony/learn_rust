use task_2791::*;

#[test]
fn public_examples() {
    assert_eq!(final_settings_trace(""), vec![]);
    assert_eq!(final_settings_trace("a=1"), vec![1u32]);
}
