use task_632::*;

#[test]
fn public_examples() {
    assert_eq!(
        border(Panel {
            width: 3,
            height: 5
        }),
        16
    );
    assert_eq!(
        border(Panel {
            width: 0,
            height: 0
        }),
        0
    );
}
