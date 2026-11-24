use task_1568::*;

#[test]
fn public_examples() {
    assert_eq!(
        feature_padding_steps(&[]),
        if cfg!(feature = "wide") {
            vec![]
        } else {
            vec![]
        }
    );
    assert_eq!(
        feature_padding_steps(&[4]),
        if cfg!(feature = "wide") {
            vec![]
        } else {
            vec![]
        }
    );
}
