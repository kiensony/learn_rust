// rust-track: handwritten task-2696
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2696;

pub fn lending_windows_threshold_crossing(input: &[u8], limit: u32) -> Option<(usize, u32)> {
    let observations: Vec<u32> = {
        trait Lending {
            type Item<'a>
            where
                Self: 'a;
            fn item(&self) -> Self::Item<'_>;
        }

        struct Window<'a>(&'a [u8]);
        impl Lending for Window<'_> {
            type Item<'a>
                = &'a [u8]
            where
                Self: 'a;
            fn item(&self) -> Self::Item<'_> {
                self.0
            }
        }
        input
            .chunks(2)
            .map(|raw| {
                let win = Window(raw);
                let s = win.item();
                s.len() as u32
            })
            .collect()
    };
    observations
        .into_iter()
        .enumerate()
        .find(|&(_, value)| value >= limit)
}
