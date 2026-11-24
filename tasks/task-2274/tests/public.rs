use task_2274::*;

#[test]
fn public_examples() {
    assert_eq!(exclusive_hotspots_peaks(&[]), vec![]);
    assert_eq!(exclusive_hotspots_peaks(&[4]), vec![4u32]);
}
