use task_410::*;

#[test]
fn example() {
    assert_eq!(
        {
            let mut v = vec![8, 3];
            (repeat_first(&mut v), v)
        },
        (Some(8), vec![8, 3, 8])
    );
}
