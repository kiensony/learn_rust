use task_374::*;

#[test]
fn public_examples() {
    assert_eq!(
        prefix_before_final_mark_slash(&String::from("path/1/12")),
        1
    );
    assert_eq!(prefix_before_final_mark_slash(&String::from("path/123")), 0);
}
