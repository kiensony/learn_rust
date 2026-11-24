use task_1563::*;

#[test]
fn public_examples() {
    assert_eq!(
        feature_padding_bounds(&[]),
        if cfg!(feature = "wide") { None } else { None }
    );
    assert_eq!(
        feature_padding_bounds(&[4]),
        if cfg!(feature = "wide") {
            Some((16u32, 16u32))
        } else {
            Some((4u32, 4u32))
        }
    );
}
