use task_2133::*;

#[test]
fn public_examples() {
    assert_eq!(histogram_bins_bounds(&[]), None);
    assert_eq!(histogram_bins_bounds(&[4]), Some((1u32, 1u32)));
}
