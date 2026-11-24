use task_2999::*;

#[test]
fn transfer_case() {
    assert_eq!(handoff(vec![7, 7, 7], 1), (vec![7], vec![7, 7]));
}

#[test]
fn every_shutdown_boundary_conserves_order_and_multiplicity() {
    let records = [0, 8, 8, 5, 8];
    for limit in 0..=records.len() + 1 {
        let (mut accepted, rejected) = handoff(records.to_vec(), limit);
        assert_eq!(accepted.len(), limit.min(records.len()));
        accepted.extend(rejected);
        assert_eq!(accepted, records);
    }
}
