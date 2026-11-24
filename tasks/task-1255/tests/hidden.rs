use task_1255::*;

#[test]
fn edge_cases() {
    assert_eq!(command_arguments_changes("-q -q -- -v"), vec![2u32]);
    assert_eq!(
        command_arguments_changes("--level=3 data --level=9"),
        vec![9u32, 4u32, 9u32]
    );
    assert_eq!(
        command_arguments_changes("a --bad --level=x b"),
        vec![1u32, 5u32, 9u32, 1u32]
    );
    assert_eq!(
        command_arguments_changes("input -v -q input --level=255"),
        vec![5u32, 2u32, 5u32, 11u32]
    );
    assert_eq!(
        command_arguments_changes("-- --level=4 x"),
        vec![2u32, 9u32, 1u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1255::TRACK_PROOF, 0x52540000 | 1255);
}
