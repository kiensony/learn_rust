use task_652::*;

#[test]
fn edge_cases() {
    assert_eq!(
        restock(
            Stock {
                units: 0,
                capacity: 0
            },
            8
        ),
        Stock {
            units: 0,
            capacity: 0
        }
    );
    assert_eq!(
        restock(
            Stock {
                units: 65530,
                capacity: 65535
            },
            50
        ),
        Stock {
            units: 65535,
            capacity: 65535
        }
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_652::TRACK_PROOF, 0x52540000 | 652);
}
