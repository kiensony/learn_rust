use task_2824::*;

#[test]
fn public_examples() {
    assert_eq!(service_sharding_peaks(&[]), vec![]);
    assert_eq!(service_sharding_peaks(&[4]), vec![0u32]);
}
