use task_929::*;

#[test]
fn public_examples() {
    assert_eq!(
        provide(Some(String::from("up")), || String::from("down")),
        "up"
    );
    assert_eq!(provide(None, || String::from("late")), "late");
}
