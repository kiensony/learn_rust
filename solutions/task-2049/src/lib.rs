// rust-track: handwritten task-2049
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2049;

pub fn polled_masks_frequent_readings(input: &[u8], keep: usize) -> Vec<u32> {
    let observations: Vec<u32> = {
        use std::future::Future;
        use std::task::{Context, Poll, Waker};
        let mut cx = Context::from_waker(Waker::noop());
        input
            .iter()
            .map(|&x| {
                let mut first = true;
                let f = std::future::poll_fn(|cx| {
                    if first {
                        first = false;
                        cx.waker().wake_by_ref();
                        Poll::Pending
                    } else {
                        Poll::Ready(u32::from(x & 7))
                    }
                });
                let mut pin = std::pin::pin!(f);
                assert!(pin.as_mut().poll(&mut cx).is_pending());
                match pin.as_mut().poll(&mut cx) {
                    Poll::Ready(v) => v,
                    Poll::Pending => unreachable!(),
                }
            })
            .collect()
    };
    let mut counts = std::collections::BTreeMap::new();
    for (index, value) in observations.into_iter().enumerate() {
        let (count, _) = counts.entry(value).or_insert((0usize, index));
        *count += 1;
    }
    let mut entries: Vec<_> = counts.into_iter().collect();
    entries.sort_unstable_by_key(|(_, (count, first))| (std::cmp::Reverse(*count), *first));
    entries
        .into_iter()
        .take(keep)
        .map(|(value, _)| value)
        .collect()
}
