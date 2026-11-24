use task_414::*;

#[test]
fn public_examples() {
    assert_eq!(
        {
            let mut data = String::from("hi:kit");
            prefix_before_final_mark_colon(&mut data);
            data
        },
        "hi"
    );
    assert_eq!(
        {
            let mut data = String::from(":é:a");
            prefix_before_final_mark_colon(&mut data);
            data
        },
        ":é"
    );
}
