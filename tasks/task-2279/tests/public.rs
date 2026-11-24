use task_2279::*;

#[test]
fn public_examples() {
    assert_eq!(exclusive_hotspots_longest_run(&[]), 0);
    assert_eq!(exclusive_hotspots_longest_run(&[4]), 1);
}
