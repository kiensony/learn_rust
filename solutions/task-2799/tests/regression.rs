use task_2799::*;

#[test]
fn transfer_case() {
    assert_eq!(
        parsed_defaults_frequent_readings("a=0;b=1;b=2;z=255 a=0;b=1;b=2;z=255", 2),
        vec![1u32]
    );
}
