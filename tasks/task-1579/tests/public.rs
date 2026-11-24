use task_1579::*;

#[test]
fn public_examples() {
    assert_eq!(
        feature_packing_longest_run(&[]),
        if cfg!(feature = "wide") { 0 } else { 0 }
    );
    assert_eq!(
        feature_packing_longest_run(&[4]),
        if cfg!(feature = "wide") { 1 } else { 1 }
    );
}
