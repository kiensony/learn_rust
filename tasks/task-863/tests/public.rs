use task_863::*;

#[test]
fn public_examples() {
    assert_eq!(is_exit(Command::Exit), true);
    assert_eq!(is_exit(Command::Write(2)), false);
}
