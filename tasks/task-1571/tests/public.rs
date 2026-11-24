use task_1571::*;

#[test]
fn public_examples() {
    assert_eq!(
        feature_packing_trace(&[]),
        if cfg!(feature = "wide") {
            vec![]
        } else {
            vec![]
        }
    );
    assert_eq!(
        feature_packing_trace(&[4]),
        if cfg!(feature = "wide") {
            vec![32u32]
        } else {
            vec![4u32]
        }
    );
}
