use task_1578::*;

#[test]
fn public_examples() {
    assert_eq!(
        feature_packing_steps(&[]),
        if cfg!(feature = "wide") {
            vec![]
        } else {
            vec![]
        }
    );
    assert_eq!(
        feature_packing_steps(&[4]),
        if cfg!(feature = "wide") {
            vec![]
        } else {
            vec![]
        }
    );
}
