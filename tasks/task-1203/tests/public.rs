use task_1203::*;

#[test]
fn example() {
    assert_eq!(
        {
            let mut n = 4;
            record(&mut n, 7);
            n
        },
        11
    );
}
