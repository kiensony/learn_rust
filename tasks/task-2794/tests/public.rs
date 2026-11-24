use task_2794::*;

#[test]
fn public_examples() {
    assert_eq!(final_settings_peaks(""), vec![]);
    assert_eq!(final_settings_peaks("a=1"), vec![1u32]);
}
