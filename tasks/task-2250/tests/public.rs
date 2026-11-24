use task_2250::*;

#[test]
fn public_examples() {
    assert_eq!(
        borrowed_windows_replay_mismatch(
            "a b c\n\r\nup\tdown",
            &[
                24864, 8290, 25120, 8291, 25354, 2573, 3338, 2677, 30064, 28681, 2404, 25711, 28535
            ]
        ),
        Some(13)
    );
    assert_eq!(borrowed_windows_replay_mismatch("", &[]), None);
}
