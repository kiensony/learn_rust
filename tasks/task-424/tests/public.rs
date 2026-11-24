use task_424::*;

#[test]
fn public_examples() {
    assert_eq!(
        {
            let mut data = String::from("path/1/12");
            prefix_before_final_mark_slash(&mut data);
            data
        },
        "1"
    );
    assert_eq!(
        {
            let mut data = String::from("path/123");
            prefix_before_final_mark_slash(&mut data);
            data
        },
        ""
    );
}
