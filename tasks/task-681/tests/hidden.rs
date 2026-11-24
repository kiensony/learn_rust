use task_681::*;

#[test]
fn edge_cases() {
    assert_eq!(step(Button::Held, false), Button::Up);
    assert_eq!(step(Button::Up, false), Button::Up);
    assert_eq!(step(Button::Held, true), Button::Held);
}

#[test]
fn submission_marker() {
    assert_eq!(task_681::TRACK_PROOF, 0x52540000 | 681);
}
