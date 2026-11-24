// rust-track: handwritten task-2676
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2676;

pub fn lending_checksum_descending_steps(input: &[u8]) -> u64 {
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
            .chunks(4)
            .map(|raw| {
                let win = Window(raw);
                let s = win.item();
                s.iter().fold(0u32, |a, &x| a ^ u32::from(x))
            })
            .collect()
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
