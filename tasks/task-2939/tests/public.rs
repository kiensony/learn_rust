use task_2939::*;

#[test]
fn public_examples() {
    assert_eq!(stability_band_longest_run(&[]), 0);
    assert_eq!(stability_band_longest_run(&[4]), 0);
}
