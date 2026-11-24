use task_1585::*;

#[test]
fn public_examples() {
    assert_eq!(
        feature_lanes_changes(&[]),
        if cfg!(feature = "wide") {
            vec![]
        } else {
            vec![]
        }
    );
    assert_eq!(
        feature_lanes_changes(&[4]),
        if cfg!(feature = "wide") {
            vec![64u32]
        } else {
            vec![4u32]
        }
    );
}
