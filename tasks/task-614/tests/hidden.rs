use task_614::*;

#[test]
fn boundaries() {
    assert_eq!(
        edge(Parcel {
            width: 9,
            height: 2
        }),
        9
    );
    assert_eq!(
        edge(Parcel {
            width: 0,
            height: 0
        }),
        0
    );
}

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 614);
}
