use task_621::*;

#[test]
fn public_examples() {
    assert_eq!(
        remainder_after_first_mark_slash(&Record {
            label: String::from("path/1/12"),
            revision: 0
        }),
        2
    );
    assert_eq!(
        remainder_after_first_mark_slash(&Record {
            label: String::from("path/123"),
            revision: 1
        }),
        1
    );
}
