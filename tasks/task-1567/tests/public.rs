use task_1567::*;

#[test]
fn public_examples() {
    assert_eq!(
        feature_padding_distinct(&[]),
        if cfg!(feature = "wide") {
            vec![]
        } else {
            vec![]
        }
    );
    assert_eq!(
        feature_padding_distinct(&[4]),
        if cfg!(feature = "wide") {
            vec![16u32]
        } else {
            vec![4u32]
        }
    );
}
