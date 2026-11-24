use task_365::*;

#[test]
fn public_examples() {
    assert_eq!(first_nonempty_field_colon(&String::from("hi:kit")), 2);
    assert_eq!(first_nonempty_field_colon(&String::from("")), 0);
}
