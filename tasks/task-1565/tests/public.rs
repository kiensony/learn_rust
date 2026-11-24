use task_1565::*;

#[test]
fn public_examples() {
    assert_eq!(
        feature_padding_changes(&[]),
        if cfg!(feature = "wide") {
            vec![]
        } else {
            vec![]
        }
    );
    assert_eq!(
        feature_padding_changes(&[4]),
        if cfg!(feature = "wide") {
            vec![16u32]
        } else {
            vec![4u32]
        }
    );
}
