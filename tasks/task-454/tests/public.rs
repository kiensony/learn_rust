use task_454::*;

#[test]
fn public_examples() {
    assert_eq!(
        {
            let mut a = [10, 3];
            let moved = transfer(&mut a, 4);
            (a, moved)
        },
        ([6, 7], 4)
    );
    assert_eq!(
        {
            let mut a = [3, 2];
            let moved = transfer(&mut a, 5);
            (a, moved)
        },
        ([0, 5], 3)
    );
}
