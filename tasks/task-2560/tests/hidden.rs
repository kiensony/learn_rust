use task_2560::{TRACK_PROOF, acknowledge};

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 2560);
}

#[test]
fn handle_empty_and_consecutive_batches() {
    for count in [0, 1, 32, 0, 12] {
        assert_eq!(acknowledge(vec![u16::MAX; count].into_boxed_slice()), count);
    }
}
