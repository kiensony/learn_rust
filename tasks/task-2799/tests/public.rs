use task_2799::*;

#[test]
fn public_examples() {
    assert_eq!(
        parsed_defaults_frequent_readings("a=0;b=1;b=2;z=255", 3),
        vec![1u32]
    );
    assert_eq!(parsed_defaults_frequent_readings("", 0), vec![]);
}
