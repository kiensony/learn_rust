use task_665::*;

#[test]
fn public_examples() {
    assert_eq!(wait_seconds(Signal::Go), 0);
    assert_eq!(wait_seconds(Signal::Wait(4)), 4);
}
