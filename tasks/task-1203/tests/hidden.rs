use task_1203::*;

#[test]
fn boundaries() {
    assert_eq!(
        {
            let mut n = 0;
            record(&mut n, 0);
            n
        },
        0
    );
    assert_eq!(
        {
            let mut n = u64::MAX;
            record(&mut n, 10);
            n
        },
        u64::MAX
    );
}

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 1203);
}
