use task_2503::*;

#[test]
fn boundaries() {
    assert_eq!(invoke(default_callback(), 0), 0);
    assert_eq!(
        {
            extern "C" fn twice(x: u16) -> u16 {
                x * 2
            }
            invoke(twice, 16)
        },
        32
    );
}

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 2503);
}
