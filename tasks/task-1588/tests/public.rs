use task_1588::*;

#[test]
fn public_examples() {
    assert_eq!(
        feature_lanes_steps(&[]),
        if cfg!(feature = "wide") {
            vec![]
        } else {
            vec![]
        }
    );
    assert_eq!(
        feature_lanes_steps(&[4]),
        if cfg!(feature = "wide") {
            vec![]
        } else {
            vec![]
        }
    );
}
