use task_1564::*;

#[test]
fn public_examples() {
    assert_eq!(
        feature_padding_peaks(&[]),
        if cfg!(feature = "wide") {
            vec![]
        } else {
            vec![]
        }
    );
    assert_eq!(
        feature_padding_peaks(&[4]),
        if cfg!(feature = "wide") {
            vec![16u32]
        } else {
            vec![4u32]
        }
    );
}
