// rust-track: handwritten task-2652
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2652;

pub fn lending_windows_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
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
    samples.into_iter().map(u64::from).sum()
}
