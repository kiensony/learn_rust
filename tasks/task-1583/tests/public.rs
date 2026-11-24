use task_1583::*;

#[test]
fn public_examples() {
    assert_eq!(
        feature_lanes_bounds(&[]),
        if cfg!(feature = "wide") { None } else { None }
    );
    assert_eq!(
        feature_lanes_bounds(&[4]),
        if cfg!(feature = "wide") {
            Some((64u32, 64u32))
        } else {
            Some((4u32, 4u32))
        }
    );
}
