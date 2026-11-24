use task_1553::*;

#[test]
fn public_examples() {
    assert_eq!(
        feature_scaling_bounds(&[]),
        if cfg!(feature = "wide") { None } else { None }
    );
    assert_eq!(
        feature_scaling_bounds(&[4]),
        if cfg!(feature = "wide") {
            Some((8u32, 8u32))
        } else {
            Some((4u32, 4u32))
        }
    );
}
