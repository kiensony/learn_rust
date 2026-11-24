use task_583::*;

#[test]
fn public_examples() {
    assert_eq!(heading("red\nblue"), "red");
    assert_eq!(heading("rain"), "rain");
}
