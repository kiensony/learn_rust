use task_622::*;

#[test]
fn public_examples() {
    assert_eq!(
        prefix_before_final_mark_slash(&Record {
            label: String::from("path/1/12"),
            revision: 0
        }),
        1
    );
    assert_eq!(
        prefix_before_final_mark_slash(&Record {
            label: String::from("hi/kit"),
            revision: 2
        }),
        4
    );
}
