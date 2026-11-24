use task_2250::*;

#[test]
fn edge_cases() {
    assert_eq!(borrowed_windows_replay_mismatch("a", &[]), None);
    assert_eq!(
        borrowed_windows_replay_mismatch(
            "a\nblue\nblue\r\nz\n",
            &[
                24842, 2658, 25196, 27765, 30053, 25866, 2658, 25196, 27765, 30053, 25869, 3338,
                2682, 31242, 777
            ]
        ),
        Some(14)
    );
    assert_eq!(
        borrowed_windows_replay_mismatch(
            "   x\té\n\ncat\n",
            &[
                8224, 8224, 8312, 30729, 2499, 50089, 43274, 2570, 2659, 25441, 24948
            ]
        ),
        Some(11)
    );
    assert_eq!(
        borrowed_windows_replay_mismatch("nine\nq\nthree\nx\n", &[9999]),
        Some(0)
    );
    assert_eq!(
        borrowed_windows_replay_mismatch(
            "a b c\n\r\nup\tdown",
            &[
                30574, 28535, 25711, 2404, 28681, 30064, 2677, 3338, 2573, 25354, 8291, 25120,
                8290, 24864
            ]
        ),
        Some(0)
    );
    assert_eq!(
        borrowed_windows_replay_mismatch("\n\nzz\n", &[2570, 2682, 31354, 31242]),
        None
    );
    assert_eq!(
        borrowed_windows_replay_mismatch(
            "a b c\n\r\nup\tdown",
            &[
                24864, 8290, 25120, 8291, 25354, 2573, 3338, 2677, 30064, 28681, 2404, 25711,
                28535, 30574, 1
            ]
        ),
        Some(14)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2250::TRACK_PROOF, 0x52540000 | 2250);
}
