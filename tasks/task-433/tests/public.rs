use task_433::*;

#[test]
fn public_examples() {
    assert_eq!(
        {
            let mut data = String::from("first\n123");
            remainder_after_first_mark_pipe(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("header\n猫|123");
            remainder_after_first_mark_pipe(&mut data);
            data
        },
        "123"
    );
}
