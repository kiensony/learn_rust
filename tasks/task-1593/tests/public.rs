use task_1593::*;

#[test]
fn public_examples() {
    assert_eq!(
        feature_pages_bounds(&[]),
        if cfg!(feature = "wide") { None } else { None }
    );
    assert_eq!(
        feature_pages_bounds(&[4]),
        if cfg!(feature = "wide") {
            Some((128u32, 128u32))
        } else {
            Some((4u32, 4u32))
        }
    );
}
