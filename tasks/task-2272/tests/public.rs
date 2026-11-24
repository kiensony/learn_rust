use task_2272::*;

#[test]
fn public_examples() {
    assert_eq!(exclusive_hotspots_total(&[]), 0);
    assert_eq!(exclusive_hotspots_total(&[4]), 4);
}
