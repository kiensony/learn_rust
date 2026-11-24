use task_418::*;

#[test]
fn public_examples() {
    assert_eq!(
        {
            let mut data = [3, 8];
            let changed = cap(&mut data, 5);
            (data, changed)
        },
        ([3, 5], 1)
    );
    assert_eq!(
        {
            let mut data = [9, 9, 1];
            let changed = cap(&mut data, 9);
            (data, changed)
        },
        ([9, 9, 1], 0)
    );
}
