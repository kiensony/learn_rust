use task_410::*;

#[test]
fn boundaries() {
    assert_eq!(
        {
            let mut v = vec![];
            (repeat_first(&mut v), v)
        },
        (None, vec![])
    );
    assert_eq!(
        {
            let mut v = vec![0];
            (repeat_first(&mut v), v)
        },
        (Some(0), vec![0, 0])
    );
}

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 410);
}
