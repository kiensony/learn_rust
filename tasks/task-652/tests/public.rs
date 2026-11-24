use task_652::*;

#[test]
fn public_examples() {
    assert_eq!(
        restock(
            Stock {
                units: 1,
                capacity: 8
            },
            4
        ),
        Stock {
            units: 5,
            capacity: 8
        }
    );
    assert_eq!(
        restock(
            Stock {
                units: 7,
                capacity: 8
            },
            4
        ),
        Stock {
            units: 8,
            capacity: 8
        }
    );
}
