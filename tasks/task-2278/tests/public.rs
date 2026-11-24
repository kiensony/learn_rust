use task_2278::*;

#[test]
fn public_examples() {
    assert_eq!(exclusive_hotspots_steps(&[]), vec![]);
    assert_eq!(exclusive_hotspots_steps(&[4]), vec![]);
}
