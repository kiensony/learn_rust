use task_681::*;

#[test]
fn public_examples() {
    assert_eq!(step(Button::Up, true), Button::Pressed);
    assert_eq!(step(Button::Pressed, true), Button::Held);
}
