use task_454::*;

#[test]
fn transfer_case() {
    assert_eq!(
        {
            let mut a = [10, 65533];
            let moved = transfer(&mut a, 7);
            (a, moved)
        },
        ([8, 65535], 2)
    );
}
