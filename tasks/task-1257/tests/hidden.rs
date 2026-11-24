use task_1257::*;

#[test]
fn edge_cases() {
    assert_eq!(command_arguments_distinct("-q -q -- -v"), vec![2u32]);
    assert_eq!(
        command_arguments_distinct("--level=3 data --level=9"),
        vec![4u32, 9u32]
    );
    assert_eq!(
        command_arguments_distinct("a --bad --level=x b"),
        vec![1u32, 5u32, 9u32]
    );
    assert_eq!(
        command_arguments_distinct("input -v -q input --level=255"),
        vec![2u32, 5u32, 11u32]
    );
    assert_eq!(
        command_arguments_distinct("-- --level=4 x"),
        vec![1u32, 2u32, 9u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1257::TRACK_PROOF, 0x52540000 | 1257);
}
