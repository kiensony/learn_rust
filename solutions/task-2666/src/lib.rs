// rust-track: handwritten task-2666
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2666;

pub fn lending_first_byte_descending_steps(input: &[u8]) -> u64 {
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
            .chunks(3)
            .map(|raw| {
                let win = Window(raw);
                let s = win.item();
                u32::from(s[0])
            })
            .collect()
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
