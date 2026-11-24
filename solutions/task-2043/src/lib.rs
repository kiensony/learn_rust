// rust-track: handwritten task-2043
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2043;

pub fn polled_costs_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
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
                        Poll::Ready(x.count_ones())
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
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
