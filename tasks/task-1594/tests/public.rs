use task_1594::*;

#[test]
fn public_examples() {
    assert_eq!(
        feature_pages_peaks(&[]),
        if cfg!(feature = "wide") {
            vec![]
        } else {
            vec![]
        }
    );
    assert_eq!(
        feature_pages_peaks(&[4]),
        if cfg!(feature = "wide") {
            vec![128u32]
        } else {
            vec![4u32]
        }
    );
}
