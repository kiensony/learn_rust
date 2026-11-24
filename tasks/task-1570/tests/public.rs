use task_1570::*;

#[test]
fn public_examples() {
    assert_eq!(
        feature_padding_reverse(&[]),
        if cfg!(feature = "wide") {
            vec![]
        } else {
            vec![]
        }
    );
    assert_eq!(
        feature_padding_reverse(&[4]),
        if cfg!(feature = "wide") {
            vec![16u32]
        } else {
            vec![4u32]
        }
    );
}
