use task_2273::*;

#[test]
fn public_examples() {
    assert_eq!(exclusive_hotspots_bounds(&[]), None);
    assert_eq!(exclusive_hotspots_bounds(&[4]), Some((4u32, 4u32)));
}
