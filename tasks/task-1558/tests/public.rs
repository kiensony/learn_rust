use task_1558::*;

#[test]
fn public_examples() {
    assert_eq!(
        feature_scaling_steps(&[]),
        if cfg!(feature = "wide") {
            vec![]
        } else {
            vec![]
        }
    );
    assert_eq!(
        feature_scaling_steps(&[4]),
        if cfg!(feature = "wide") {
            vec![]
        } else {
            vec![]
        }
    );
}
