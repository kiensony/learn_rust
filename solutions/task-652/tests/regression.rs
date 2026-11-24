use task_652::*;

#[test]
fn transfer_case() {
    assert_eq!(
        restock(
            Stock {
                units: 9,
                capacity: 5
            },
            0
        ),
        Stock {
            units: 5,
            capacity: 5
        }
    );
}
