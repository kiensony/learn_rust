use task_1581::*;

#[test]
fn public_examples() {
    assert_eq!(
        feature_lanes_trace(&[]),
        if cfg!(feature = "wide") {
            vec![]
        } else {
            vec![]
        }
    );
    assert_eq!(
        feature_lanes_trace(&[4]),
        if cfg!(feature = "wide") {
            vec![64u32]
        } else {
            vec![4u32]
        }
    );
}
