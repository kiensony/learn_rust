use task_1258::*;

#[test]
fn edge_cases() {
    assert_eq!(
        command_arguments_steps("-q -q -- -v"),
        vec![0u32, 0u32, 0u32]
    );
    assert_eq!(
        command_arguments_steps("--level=3 data --level=9"),
        vec![5u32, 5u32]
    );
    assert_eq!(
        command_arguments_steps("a --bad --level=x b"),
        vec![4u32, 4u32, 8u32]
    );
    assert_eq!(
        command_arguments_steps("input -v -q input --level=255"),
        vec![3u32, 0u32, 3u32, 6u32]
    );
    assert_eq!(command_arguments_steps("-- --level=4 x"), vec![7u32, 8u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1258::TRACK_PROOF, 0x52540000 | 1258);
}
