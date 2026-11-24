use task_1573::*;

#[test]
fn public_examples() {
    assert_eq!(
        feature_packing_bounds(&[]),
        if cfg!(feature = "wide") { None } else { None }
    );
    assert_eq!(
        feature_packing_bounds(&[4]),
        if cfg!(feature = "wide") {
            Some((32u32, 32u32))
        } else {
            Some((4u32, 4u32))
        }
    );
}
