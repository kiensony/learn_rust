use task_2519::*;

#[test]
fn edge_cases() {
    assert_eq!(abi_flags_longest_run(&[9, 2, 2, 8, 0, 5]), 4);
    assert_eq!(abi_flags_longest_run(&[0, 0, 0]), 3);
    assert_eq!(abi_flags_longest_run(&[255, 1, 255, 2]), 4);
    assert_eq!(
        abi_flags_longest_run(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        6
    );
    assert_eq!(abi_flags_longest_run(&[17, 17, 1, 0, 2]), 3);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2519::TRACK_PROOF, 0x52540000 | 2519);
}
