use task_1260::*;

#[test]
fn edge_cases() {
    assert_eq!(
        command_arguments_reverse("-q -q -- -v"),
        vec![2u32, 2u32, 2u32, 2u32]
    );
    assert_eq!(
        command_arguments_reverse("--level=3 data --level=9"),
        vec![9u32, 4u32, 9u32]
    );
    assert_eq!(
        command_arguments_reverse("a --bad --level=x b"),
        vec![1u32, 9u32, 5u32, 1u32]
    );
    assert_eq!(
        command_arguments_reverse("input -v -q input --level=255"),
        vec![11u32, 5u32, 2u32, 2u32, 5u32]
    );
    assert_eq!(
        command_arguments_reverse("-- --level=4 x"),
        vec![1u32, 9u32, 2u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1260::TRACK_PROOF, 0x52540000 | 1260);
}
