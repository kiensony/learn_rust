use task_1254::*;

#[test]
fn edge_cases() {
    assert_eq!(command_arguments_peaks("-q -q -- -v"), vec![2u32]);
    assert_eq!(
        command_arguments_peaks("--level=3 data --level=9"),
        vec![9u32]
    );
    assert_eq!(
        command_arguments_peaks("a --bad --level=x b"),
        vec![1u32, 5u32, 9u32]
    );
    assert_eq!(
        command_arguments_peaks("input -v -q input --level=255"),
        vec![5u32, 11u32]
    );
    assert_eq!(command_arguments_peaks("-- --level=4 x"), vec![2u32, 9u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1254::TRACK_PROOF, 0x52540000 | 1254);
}
