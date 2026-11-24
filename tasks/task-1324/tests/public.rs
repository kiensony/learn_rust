use task_1324::*;

#[test]
fn public_examples() {
    assert_eq!(flag_fields_peaks(""), vec![]);
    assert_eq!(flag_fields_peaks("1"), vec![1u32]);
}
