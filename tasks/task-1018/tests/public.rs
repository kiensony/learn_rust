use task_1018::*;

#[test]
fn public_examples() {
    assert_eq!(
        prefix_before_final_mark_slash(["path/1/12", "path/123"]),
        [1, 0]
    );
    assert_eq!(
        prefix_before_final_mark_slash([String::from("path/123"), String::from("hi/kit")]),
        [0, 2]
    );
}
