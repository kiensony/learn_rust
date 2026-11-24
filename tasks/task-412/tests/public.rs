use task_412::*;

#[test]
fn public_examples() {
    assert_eq!(
        {
            let mut data = String::from("hi:kit");
            marks_removed_at_both_ends_colon(&mut data);
            data
        },
        "hi:kit"
    );
    assert_eq!(
        {
            let mut data = String::from(":é:a");
            marks_removed_at_both_ends_colon(&mut data);
            data
        },
        "é:a"
    );
}
