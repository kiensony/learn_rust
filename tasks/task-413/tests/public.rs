use task_413::*;

#[test]
fn public_examples() {
    assert_eq!(
        {
            let mut data = String::from("hi:kit");
            remainder_after_first_mark_colon(&mut data);
            data
        },
        "kit"
    );
    assert_eq!(
        {
            let mut data = String::from(":é:a");
            remainder_after_first_mark_colon(&mut data);
            data
        },
        "é:a"
    );
}
