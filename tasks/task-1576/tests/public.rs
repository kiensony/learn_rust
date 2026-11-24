use task_1576::*;

#[test]
fn public_examples() {
    assert_eq!(
        feature_packing_descending_steps(&[]),
        if cfg!(feature = "wide") { 0 } else { 0 }
    );
    assert_eq!(
        feature_packing_descending_steps(&[4]),
        if cfg!(feature = "wide") { 0 } else { 0 }
    );
}
