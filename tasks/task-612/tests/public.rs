use task_612::*;

#[test]
fn public_examples() {
    assert_eq!(
        prefix_before_final_mark_colon(&Record {
            label: String::from("hi:kit"),
            revision: 0
        }),
        2
    );
    assert_eq!(
        prefix_before_final_mark_colon(&Record {
            label: String::from(":é:a"),
            revision: 1
        }),
        4
    );
}
