use task_1574::*;

#[test]
fn public_examples() {
    assert_eq!(
        feature_packing_peaks(&[]),
        if cfg!(feature = "wide") {
            vec![]
        } else {
            vec![]
        }
    );
    assert_eq!(
        feature_packing_peaks(&[4]),
        if cfg!(feature = "wide") {
            vec![32u32]
        } else {
            vec![4u32]
        }
    );
}
