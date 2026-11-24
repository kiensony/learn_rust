use task_554::*;

#[test]
fn public_examples() {
    assert_eq!(puzzle_cells("A·Z"), (2, 1));
    assert_eq!(puzzle_cells("猫··虎"), (2, 2));
}
