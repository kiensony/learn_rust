use task_423::*;

#[test]
fn public_examples() {
    assert_eq!(
        {
            let mut data = String::from("path/1/12");
            remainder_after_first_mark_slash(&mut data);
            data
        },
        "12"
    );
    assert_eq!(
        {
            let mut data = String::from("path/123");
            remainder_after_first_mark_slash(&mut data);
            data
        },
        ""
    );
}
