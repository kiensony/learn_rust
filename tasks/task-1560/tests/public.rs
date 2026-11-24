use task_1560::*;

#[test]
fn public_examples() {
    assert_eq!(
        feature_scaling_reverse(&[]),
        if cfg!(feature = "wide") {
            vec![]
        } else {
            vec![]
        }
    );
    assert_eq!(
        feature_scaling_reverse(&[4]),
        if cfg!(feature = "wide") {
            vec![8u32]
        } else {
            vec![4u32]
        }
    );
}
