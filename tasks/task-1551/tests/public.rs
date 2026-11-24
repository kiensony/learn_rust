use task_1551::*;

#[test]
fn public_examples() {
    assert_eq!(
        feature_scaling_trace(&[]),
        if cfg!(feature = "wide") {
            vec![]
        } else {
            vec![]
        }
    );
    assert_eq!(
        feature_scaling_trace(&[4]),
        if cfg!(feature = "wide") {
            vec![8u32]
        } else {
            vec![4u32]
        }
    );
}
